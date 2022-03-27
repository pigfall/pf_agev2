use crate::{scene::Scene, render::RenderTrait};
use std::collections::HashMap;

pub struct App<Render:RenderTrait> {
    scenes: HashMap<String,Scene<Render>>,
    cur_scene:Scene<Render>,
}



impl<Render:RenderTrait> App<Render>{
    pub fn new(entry_scene:Scene<Render>)->Self{
        return Self{
            scenes: HashMap::new(),
            cur_scene:entry_scene,
        }
    }

    pub fn one_frame(&mut self, dt:f64){
        self.cur_scene.one_frame(dt);
    }

    pub fn render(&mut self,render:&mut Render){
        self.cur_scene.render(render);
    }
}
