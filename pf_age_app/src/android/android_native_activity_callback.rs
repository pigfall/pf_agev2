use pf_age_ndk::{ANativeActivity,ANativeWindow,AInputQueue};

pub unsafe extern "C" fn on_native_window_created(activity_raw: *mut ANativeActivity,window_raw: *mut ANativeWindow){
    todo!("");
}

pub unsafe extern "C" fn  on_native_window_destroyed(activity_raw: *mut ANativeActivity,window_raw: *mut ANativeWindow){
    todo!("");
}

pub unsafe extern "C" fn on_input_queue_created(activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){
    todo!("");
}

pub unsafe extern "C" fn on_input_queue_destroyed(activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){
    todo!("");
}


    //callbacks.onInputQueueCreated = Some(on_input_queue_created);
    //callbacks.onInputQueueDestroyed = Some(on_input_queue_destroyed);