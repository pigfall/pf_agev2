use crate::{scene::Scene, render::RenderTrait};
use std::collections::HashMap;

pub struct App {
    scenes: HashMap<String,Scene>,
    cur_scene:Scene,
}



impl App{
    pub fn new(entry_scene:Scene)->Self{
        return Self{
            scenes: HashMap::new(),
            cur_scene:entry_scene,
        }
    }

    pub fn one_frame(&mut self, dt:f64){
        self.cur_scene.one_frame(dt);
    }

    pub fn render(&mut self,render:&mut dyn RenderTrait){
        self.cur_scene.render(render);
    }
}
