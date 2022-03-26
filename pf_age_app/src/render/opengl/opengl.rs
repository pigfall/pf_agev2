use crate::render::RenderTrait;
use std::os::raw::c_void;
use pf_age_third_party::log::info;

pub struct GLRender{

}

impl GLRender{
    pub fn new()->GLRender{
        GLRender {  }
    }
}

impl RenderTrait for GLRender{
     fn on_window_create(&mut self, window_ptr: *mut c_void){
         info!("TODO opengl on_window_create");
     }
    fn on_window_destroy(&mut self, window_ptr: *mut c_void){
         info!("TODO opengl on window destroy");
    }
}