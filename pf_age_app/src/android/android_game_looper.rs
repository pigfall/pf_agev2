use pf_age_ndk::{ANativeWindow, AInputQueue};
use pf_age_third_party::log::info;

pub struct GameLooper{

}

impl GameLooper {
    pub fn loop_run(&self){
        loop{
            self.pre_handle_android_activitiy_evs();
            //app.one_frame()
        }
    }
    fn pre_handle_android_activitiy_evs(&self){

    }

    pub fn set_window_raw(&mut self,window_raw: *mut ANativeWindow){
        info!("TODO set window");
    }

    pub fn destroy_window(&mut self,window_raw: *mut ANativeWindow){
        info!("TODO destroy window");
    }

    pub fn set_input_queue(&mut self,queue:*mut AInputQueue){
        info!("TODO set input queue");
    }
    pub fn destroy_input_queue(&mut self,queue:*mut AInputQueue){
        info!("TODO destroy input queue");
    }
}