use crate::{scene::Scene};
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

    pub fn frame_advance(&mut self, dt:f64){
        //println!(" frame_advancing");
    }
}
