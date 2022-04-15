
use crate::render::{RenderTrait,Renderer};

use super::android_game_looper::GameLooper;

// pub static mut game_looper: Option<GameLooper> = None;
 pub static mut game_looper: *mut GameLooper<Renderer> = std::ptr::null_mut();
