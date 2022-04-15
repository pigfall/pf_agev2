pub mod android;
pub use android::*;

mod android_native_activity_callback;
mod android_global_game_looper;
mod android_game_looper;
pub mod bevy_plugin;
pub mod global_android_native_activity;
pub use global_android_native_activity::global_android_native_activity;
pub mod android_callback_mq;
pub use android_callback_mq::AndroidCallbackMQ;

pub mod android_bevy;
pub use android_bevy::android_main_bevy;
