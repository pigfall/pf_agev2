
use super::android_game_looper::GameLooper;

// pub static mut game_looper: Option<GameLooper> = None;
 pub static mut game_looper: *mut GameLooper = std::ptr::null_mut();
