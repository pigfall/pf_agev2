use pf_age_ndk::{ANativeWindow,AInputQueue};
pub enum Event{
    AndroidActivityEvent,
}

#[derive(Debug)]
pub enum AndroidActivityEvent {
    WindowCreate(*mut ANativeWindow),
    WindowDestroy,
    InputQueueCreated(*mut AInputQueue),
    onInputQueueDestroyed,
}