use fyrox::renderer::{Renderer as FyroxRenderer};
use fyrox::engine::resource_manager::{ResourceManager};
use pf_age_third_party::glow;
use std::os::raw::c_void;
use crate::App;

use crate::render::RenderTrait;

pub struct Renderer{
    renderer: FyroxRenderer,
}

impl Renderer{
    pub fn entry_new(
        glow_ctx:glow::Context,
        frame_size:(u32,u32),
        resource_mgr: &ResourceManager,
        )->Self{

        // init frame_size on window created

        Self{
            renderer: FyroxRenderer::new(
            glow_ctx,
            frame_size,
            resource_mgr,
            ).unwrap()
        }
    }
}

impl  RenderTrait for Renderer{
    fn on_window_create(&mut self, window_ptr: *mut c_void){
        todo!()
    }

    fn on_window_destroy(&mut self, window_ptr: *mut c_void){
        todo!()
    }

    fn render_frame(&mut self,app:&mut App){
        todo!()
    }
}
