use super::android::global_android_native_activity;
use super::android_callback_mq::AndroidCallbackMQ;
use bevy::prelude::*;
use pf_age_ndk::ANativeActivity;

pub struct AndroidNativeAcitivityPlugin {}

impl Plugin for AndroidNativeAcitivity {
    fn build(&self, app: &mut App) -> Self {
        // add native_acitivity to resource
        unsafe {
            if (global_android_native_activity.is_null()) {
                panic!("should set  global_android_native_activity before build the bevy app");
            }
            app.insert_resource(global_android_native_activity);

            let activity_ptr: *mut ANativeActivity =
                NonNull::new(global_android_native_activity).expect("activity_raw_ptr is nil");
            // set native activity callback
            let callbacks = activity_ptr
                .as_mut()
                .callbacks
                .as_mut()
                .expect("activity callback is nil");
        }

        // set runner
        todo!("");
    }
}
