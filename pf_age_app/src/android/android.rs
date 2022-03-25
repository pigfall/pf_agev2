use crate::android::android_game_looper::GameLooper;
use crate::android::android_global_game_looper::game_looper;
use crate::app::App;
use pf_age_third_party::android_logger;
use pf_age_third_party::log;
use pf_age_third_party::log::{info};
use pf_age_ndk::{ANativeActivity};
use std::ptr::NonNull;
use std::os::raw::c_void;
use std::thread;


use super::android_native_activity_callback::{
    on_native_window_created,
    on_native_window_destroyed,
    on_input_queue_created,
    on_input_queue_destroyed,
};
use super::android_global_game_looper;

pub unsafe fn android_platform_entry(activity_raw_ptr: *mut ANativeActivity,saved_state: *mut c_void,saved_size: usize,build_app:fn()->App){
    init_android_logger("pf_age");
    info!("⌛ register native activity callback");
    let mut activity_ptr = NonNull::new(activity_raw_ptr).expect("activity_raw_ptr is nil");
    let  callbacks = activity_ptr.as_mut().callbacks.as_mut().expect("activity callback is nil");
    callbacks.onNativeWindowCreated  = Some(on_native_window_created);
    callbacks.onNativeWindowDestroyed = Some(on_native_window_destroyed);
    //callbacks.onWindowFocusChanged =Some(on_native_window_focus_changed);
    callbacks.onInputQueueCreated = Some(on_input_queue_created);
    callbacks.onInputQueueDestroyed = Some(on_input_queue_destroyed);

    game_looper =Box::into_raw(Box::new(GameLooper{}));

    
    // TODO run game loop in new thread
    thread::spawn(||{
        (*game_looper).loop_run();
    });
}

pub fn init_android_logger(tag: &str){
    android_logger::init_once(
        android_logger::Config::default().
        with_min_level(log::Level::Trace). // NOTE: must need min level
        with_tag(tag),
        );
}
