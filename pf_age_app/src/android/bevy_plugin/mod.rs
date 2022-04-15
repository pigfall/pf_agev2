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
    }
}

