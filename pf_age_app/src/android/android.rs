use crate::android::android_game_looper::GameLooper;
use crate::android::android_global_game_looper::game_looper;
use crate::android::global_android_native_activity;
use crate::app::App;
use crate::render::GLRender;
use pf_age_ndk::ANativeActivity;
use pf_age_third_party::android_logger;
use pf_age_third_party::libc;
use pf_age_third_party::log;
use pf_age_third_party::log::{error, info};
use std::os::raw::c_void;
use std::os::unix::io::FromRawFd;
use std::os::unix::prelude::RawFd;
use std::ptr::NonNull;
use std::thread;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::ffi::{CStr,CString};

use super::android_global_game_looper;
use super::android_native_activity_callback::{
    on_input_queue_created, on_input_queue_destroyed, on_native_window_created,
    on_native_window_destroyed,
};

pub unsafe fn android_platform_entry(
    activity_raw_ptr: *mut ANativeActivity,
    saved_state: *mut c_void,
    saved_size: usize,
    build_app: fn() -> App,
) {
    init_android_logger("pf_age");
    info!("⌛ register native activity callback");
    let mut activity_ptr = NonNull::new(activity_raw_ptr).expect("activity_raw_ptr is nil");
    let callbacks = activity_ptr
        .as_mut()
        .callbacks
        .as_mut()
        .expect("activity callback is nil");
    callbacks.onNativeWindowCreated = Some(on_native_window_created);
    callbacks.onNativeWindowDestroyed = Some(on_native_window_destroyed);
    //callbacks.onWindowFocusChanged =Some(on_native_window_focus_changed);
    callbacks.onInputQueueCreated = Some(on_input_queue_created);
    callbacks.onInputQueueDestroyed = Some(on_input_queue_destroyed);

    game_looper = Box::into_raw(Box::new(GameLooper::new(build_app(), GLRender::new())));

    let mut logpipe: [RawFd; 2] = Default::default();
    libc::pipe(logpipe.as_mut_ptr());
    libc::dup2(logpipe[1], libc::STDOUT_FILENO);
    libc::dup2(logpipe[1], libc::STDERR_FILENO);
    thread::spawn(move || {
        // let tag = CStr::from_bytes_with_nul(b"pf_age_logger\0").unwrap();

        let file = File::from_raw_fd(logpipe[0]);
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            if let Ok(len) = reader.read_line(&mut buffer) {
                if len == 0 {
                    break;
                } else if let Ok(msg) = CString::new(buffer.clone()) {
                    error!("{:?}", msg);
                    // android_logger(Level::Info, tag, &msg);
                }
            }
        }
    });

    // TODO run game loop in new thread
    thread::spawn(|| {
        (*game_looper).loop_run();
    });
}



pub fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default().
        with_min_level(log::Level::Trace). // NOTE: must need min level
        with_tag(tag),
    );
}

pub static mut bevy_android_plugin_inited:bool = false;
