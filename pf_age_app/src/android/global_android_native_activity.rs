use pf_age_ndk::ANativeActivity;

pub static mut global_android_native_activity:*mut ANativeActivity = std::ptr::null_mut();
