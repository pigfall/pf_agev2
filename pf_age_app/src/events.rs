use pf_age_ndk::{ANativeWindow,AInputQueue,InputEvent};
#[derive(Debug)]
pub enum Event{
    AndroidActivityEvent,
    AndroidInputEvent(AndroidInputWrapper),
}

#[derive(Debug)]
pub enum AndroidActivityEvent {
    WindowCreate(ANativeWindowWrapper),
    WindowDestroy,
    InputQueueCreated(AInputQueueWrapper),
    onInputQueueDestroyed,
}

#[derive(Debug)]
pub struct AInputQueueWrapper{
    pub queue: *mut AInputQueue,
}

unsafe impl Send for AInputQueueWrapper{}
unsafe impl Sync for AInputQueueWrapper{}

#[derive(Debug)]
pub struct ANativeWindowWrapper{
    pub window: *mut ANativeWindow,
}

unsafe impl Send for ANativeWindowWrapper{}
unsafe impl Sync for ANativeWindowWrapper{}

#[derive(Debug)]
pub struct AndroidInputWrapper{
    pub ev:InputEvent
}

unsafe impl Send for AndroidInputWrapper{}
unsafe impl Sync for AndroidInputWrapper{}