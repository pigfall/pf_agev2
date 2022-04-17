use crate::render::Renderer;
use std::os::raw::c_void;
use pf_egl::{Egl14};

pub struct EglRenderer{
    egl: Egl14
}

impl EglRenderer{
    pub fn entry_load()->Result<Self,String>{
        let mut egl = Egl14::entry_load()?;
        egl.entry_once_init()?;
        return Ok(
                Self{
                    egl:egl,
                }
            );
    }

}

impl Renderer for EglRenderer{
     fn on_window_create(&mut self, window_ptr: *mut c_void){
         let surface = self.egl.entry_create_surface(window_ptr).unwrap();
         self.egl.attach_surface_to_ctx(surface).unwrap();
     }
    fn on_window_destroy(&mut self, window_ptr: *mut c_void){
        self.egl.destroy_cur_surface().unwrap();
    }
}
