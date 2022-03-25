use pf_age_ndk::{ANativeActivity,ANativeWindow,AInputQueue};

use crate::android::android_global_game_looper::game_looper;

pub unsafe extern "C" fn on_native_window_created(activity_raw: *mut ANativeActivity,window_raw: *mut ANativeWindow){
    (*game_looper).set_window_raw(window_raw);
}

pub unsafe extern "C" fn  on_native_window_destroyed(activity_raw: *mut ANativeActivity,window_raw: *mut ANativeWindow){
    (*game_looper).destroy_window(window_raw);
}

pub unsafe extern "C" fn on_input_queue_created(activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){
    (*game_looper).set_input_queue(queue);
}

pub unsafe extern "C" fn on_input_queue_destroyed(activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){
    (*game_looper).destroy_input_queue(queue);
}


    //callbacks.onInputQueueCreated = Some(on_input_queue_created);
    //callbacks.onInputQueueDestroyed = Some(on_input_queue_destroyed);