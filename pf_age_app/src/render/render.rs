use std::os::raw::c_void;
use crate::app::App;

pub trait Renderer {
     fn on_window_create(&mut self, window_ptr: *mut c_void);
    fn on_window_destroy(&mut self, window_ptr: *mut c_void);
    fn render_frame(&mut self,app:&App);
}
