use crate::render::Renderer;
use std::os::raw::c_void;
pub struct EglRenderer{

}

impl EglRenderer{
    pub fn new()->Self{
        todo!("")
    }
}

impl Renderer for EglRenderer{
     fn on_window_create(&mut self, window_ptr: *mut c_void){
         todo!("");
     }
    fn on_window_destroy(&mut self, window_ptr: *mut c_void){
        todo!("")
    }
}
