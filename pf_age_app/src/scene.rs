use crate::components::RenderFn;
use pf_age_third_party::egl::Downcast;
use pf_age_third_party::legion::{self, IntoQuery, World};
use std::any::Any;

use crate::render::{GLRender, RenderTrait};
pub struct Scene {
    // pub one_frame_fn: fn(data: &mut Box<dyn Any>,dt:f64),
    // pub data: Box<dyn Any>,
    //pub render_fn: fn(data:&mut Box<dyn Any>,render :&mut Render),
    pub start_up_fn: fn(&mut legion::World),
    pub ecs_world: legion::World,
    pub ecs_schedule: legion::Schedule,
    ecs_resources: legion::Resources,
}

impl Scene {
    pub fn new(
        start_up_fn: fn(&mut legion::World),
        ecs_schedule: legion::Schedule,
        resource: legion::Resources,
    ) -> Self {
        let mut world = legion::World::default();
        start_up_fn(&mut world);
        return Self {
            start_up_fn: start_up_fn,
            ecs_world: world,
            ecs_schedule: ecs_schedule,
            ecs_resources: resource,
        };
    }
    pub fn add_start_up_system(&mut self, f: fn(&mut legion::World)) {
        self.start_up_fn = f;
    }
}

impl Scene {
    pub fn one_frame(&mut self, dt: f64) {
        self.ecs_schedule
            .execute(&mut self.ecs_world, &mut self.ecs_resources);
    }
    pub fn render(&mut self, render: &mut dyn RenderTrait) {
        let mut query = legion::Read::<RenderFn>::query();
        let r= render.as_any().downcast_mut::<GLRender>().expect("TODO exptect gl");
        if let Some(gl_fns) =  &mut r.gl_fn_wrapper{
                for (renderfn) in query.iter(&self.ecs_world) {
                    (renderfn.f)(&mut gl_fns.gl_raw_fns);
                };
        };
        if let Some(surface )= r.surface{
            let egl_wrapper   =r.egl_wrapper.as_mut().expect("TODO");
            egl_wrapper.egl_ins.swap_buffers(egl_wrapper.display,surface).unwrap();
        };
    }
}
