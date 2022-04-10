use crate::android::global_android_native_activity;
use pf_age_ndk::ANativeWindow;
use crate::events::{AndroidActivityEvent,ANativeWindowWrapper,AInputQueueWrapper};
use crate::android::android_callback_mq::AndroidCallbackMQ;
use bevy::prelude::*;
use pf_age_ndk::ANativeActivity;
use crate::android::bevy_android_plugin_inited;
use std::ptr::NonNull;
use pf_age_ndk::{AInputQueue,};

pub struct AndroidNativeAcitivityPlugin {}

pub struct AndroidNativeAcitivityWrapper{
    raw: *mut ANativeActivity,
}

unsafe impl Send for AndroidNativeAcitivityWrapper{}
unsafe impl Sync for AndroidNativeAcitivityWrapper{}

impl Plugin for AndroidNativeAcitivityPlugin {
    fn build(&self, app: &mut App)  {
        unsafe{
            if global_android_native_activity_mq.is_null(){
                global_android_native_activity_mq = Box::into_raw(Box::new(AndroidCallbackMQ::new()));
            }
        };
        // add native_acitivity to resource
        unsafe {
            if (global_android_native_activity.is_null()) {
                panic!("should set  global_android_native_activity before build the bevy app");
            }
            app.insert_resource(AndroidNativeAcitivityWrapper{raw:global_android_native_activity});

            let mut activity_ptr =
                NonNull::new(global_android_native_activity).expect("activity_raw_ptr is nil");
            // set native activity callback
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
        }

        // set runner
        app.set_runner(|app|{
            loop{
                let need_handle_native_event = pre_handle_android_native_event();
                // todo
                //
                if need_handle_native_event{
                    post_handle_android_native_event();
                }
            }
        });

        unsafe{
            bevy_android_plugin_inited = true;
        };
    }
}

fn pre_handle_android_native_event()->bool{
    unsafe{
        let msgs = (*global_android_native_activity_mq).read_msg();
        if msgs.len() > 0{
            for msg in msgs{
                // todo
            }
            return true;
        }
        return false;
    }
}

fn post_handle_android_native_event(){
    unsafe{
        (*global_android_native_activity_mq).notify_mq_msg_handled();
    };
}

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

