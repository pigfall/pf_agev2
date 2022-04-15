use crate::android:: init_android_logger;
use std::{thread, time};
use bevy::app::PluginGroupBuilder;
use crate::events::*;
use crate::android::AndroidCallbackMQ;
use pf_age_ndk::*;
use crate::android::android_game_looper::GameLooper;
use crate::android::android_global_game_looper::game_looper;
use crate::android::global_android_native_activity;
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
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::ffi::{CStr,CString};

use bevy::input::InputPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::hierarchy::HierarchyPlugin;
use bevy::transform::TransformPlugin;
use bevy::core::CorePlugin;
use bevy::asset::AssetPlugin;
use bevy::render::RenderPlugin;


pub unsafe fn android_main_bevy(
    activity_raw_ptr: *mut ANativeActivity,
    saved_state: *mut c_void,
    saved_size: usize,
    build_app: fn()->bevy::prelude::App ,
) {
    init_android_logger("pf_age");
    info!("⌛ register native activity callback");

    global_android_native_activity = activity_raw_ptr;
    global_android_native_activity_mq = unsafe{
        Box::into_raw(Box::new(AndroidCallbackMQ::new()))
    };

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

    // TODO run game loop in new thread
    thread::spawn(move || {
        info!("entrying");
        loop{
            let need_update = pre_handle_android_native_event();
            if need_update {
                global_android_native_activity_mq.as_mut().unwrap().notify_mq_msg_handled();
            }
            if !global_android_native_window.is_null(){
                break;
            }
        };
        info!("building app");
        build_app().run();
    });
    //while !bevy_android_plugin_inited{
    //    info!("waiting bevy_android_plugin_inited")
    //}
}

pub struct DefaultPlugins{

}

impl bevy::prelude::PluginGroup for DefaultPlugins{
    fn build(&mut self,group:&mut  PluginGroupBuilder){
        //group.add(CorePlugin::default());
        //group.add(TransformPlugin::default());
        //group.add(HierarchyPlugin::default());
        //group.add(DiagnosticsPlugin::default());
        //group.add(InputPlugin::default());
        //group.add(WindowPlugin::default());
        //group.add(AssetPlugin::default());
        group.add(GameRunner{});
    }
}

pub struct GameRunner{

}

impl bevy::prelude::Plugin for GameRunner{
    fn build(&self,app:&mut bevy::prelude::App){
        app.add_plugin(CorePlugin::default());
        app.add_plugin(TransformPlugin::default());
        app.add_plugin(HierarchyPlugin::default());
        app.add_plugin(DiagnosticsPlugin::default());
        app.add_plugin(InputPlugin::default());
        app.add_plugin(WindowPlugin{});
        app.add_plugin(AssetPlugin::default());
        app.add_plugin(RenderPlugin::default());

        app.set_runner(|mut app|{
            loop{
                thread::sleep(time::Duration::from_secs(2));
                info!("alive");
                app.update();
            };
        });
    }
}

pub struct WindowPlugin{

}

impl bevy::prelude::Plugin for WindowPlugin{
    fn build(&self,app: &mut bevy::prelude::App){
        use bevy::window::*;
        app.init_resource::<bevy::window::Windows>();
        app.add_event::<WindowResized>()
            .add_event::<CreateWindow>()
            .add_event::<WindowCreated>()
            .add_event::<WindowCloseRequested>()
            .add_event::<RequestRedraw>()
            .add_event::<CloseWindow>()
            .add_event::<CursorMoved>()
            .add_event::<CursorEntered>()
            .add_event::<CursorLeft>()
            .add_event::<ReceivedCharacter>()
            .add_event::<WindowFocused>()
            .add_event::<WindowScaleFactorChanged>()
            .add_event::<WindowBackendScaleFactorChanged>()
            .add_event::<FileDragAndDrop>()
            .add_event::<WindowMoved>();

        let mut windows = app.world.get_resource_mut::<bevy::window::Windows>().unwrap();
        let mut handle = raw_window_handle::AndroidNdkHandle::empty();
        unsafe{
            handle.a_native_window = global_android_native_window as _;
        };
        let window = bevy::window::Window::new(
            bevy::window::WindowId::primary(),
            &bevy::window::WindowDescriptor::default(),
            500,
            500,
            1.0,
            None,
            raw_window_handle::RawWindowHandle::AndroidNdk(handle),
            );
        windows.add(window);
    }
}





pub static mut global_android_native_window: *mut ANativeWindow = std::ptr::null_mut();

pub static mut global_android_native_activity_mq:* mut AndroidCallbackMQ=std::ptr::null_mut();


pub unsafe extern "C" fn on_native_window_created(activity_raw: *mut ANativeActivity,window_raw: *mut ANativeWindow){
    global_android_native_activity_mq.as_mut().unwrap().push_msg_then_wait_handled(AndroidActivityEvent::WindowCreate(ANativeWindowWrapper{window:window_raw}));
}

pub unsafe extern "C" fn  on_native_window_destroyed(activity_raw: *mut ANativeActivity,window_raw: *mut ANativeWindow){
    global_android_native_activity_mq.as_mut().unwrap().push_msg_then_wait_handled(AndroidActivityEvent::WindowDestroy);
}

pub unsafe extern "C" fn on_input_queue_created(activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){
    global_android_native_activity_mq.as_mut().unwrap().push_msg_then_wait_handled(AndroidActivityEvent::InputQueueCreated(AInputQueueWrapper{queue:queue}));
}

pub unsafe extern "C" fn on_input_queue_destroyed(activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){
    global_android_native_activity_mq.as_mut().unwrap().push_msg_then_wait_handled(AndroidActivityEvent::onInputQueueDestroyed);
}

fn pre_handle_android_native_event()->bool{
    unsafe{
        let msgs = (*global_android_native_activity_mq).read_msg();
        if msgs.len() > 0{
            for msg in msgs{
                match msg{
                    AndroidActivityEvent::WindowCreate(wrapper)=>{
                        global_android_native_window =wrapper.window;
                    },
                    _=>{},
                }
                // todo
            }
            return true;
        }
        return false;
    }
}

