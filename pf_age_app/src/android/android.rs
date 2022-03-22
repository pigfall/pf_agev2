use crate::app::App;
pub use pf_age_third_party::android_logger;
pub use pf_age_third_party::log;
pub use pf_age_third_party::log::{info};
use std::os::raw::c_void;

pub fn android_platform_entry(activity: *mut c_void,saved_state: *mut c_void,saved_size: usize,build_app:fn()->App){
    info!("⌛ register native activity callback")
}

pub fn init_android_logger(tag: &str){
    android_logger::init_once(
        android_logger::Config::default().
        with_min_level(log::Level::Trace). // NOTE: must need min level
        with_tag(tag),
        );
}

struct GameLooper{

}

impl GameLooper {
    fn loop_run(&self){
        loop{
            pre_handle_android_activitiy_evs();
            app.one_frame()
        }
    }
}
