use std::os::raw::c_void;
use std::any::Any;

pub trait RenderTrait{
     fn on_window_create(&mut self, window_ptr: *mut c_void);
    fn on_window_destroy(&mut self, window_ptr: *mut c_void);
     fn as_any(&mut self) -> &mut dyn Any;
}
