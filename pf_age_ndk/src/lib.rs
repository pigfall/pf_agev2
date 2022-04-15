#[cfg(target_os="android")]
pub use ndk_sys::{
    ANativeActivity,
    ANativeWindow, 
    AInputQueue,
    AInputQueue_getEvent,
    AInputQueue_finishEvent,
};

#[cfg(target_os="android")]
pub use ndk::event::InputEvent;
