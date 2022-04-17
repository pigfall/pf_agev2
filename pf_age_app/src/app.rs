use crate::{game_scene::Scene};
use std::collections::HashMap;
use pf_age_third_party::legion::{World};

pub struct App {
    scenes: HashMap<String,Scene>,
    cur_scene:Scene,
    world: World,
}



impl App{
    pub fn new(entry_scene:Scene)->Self{
        return Self{
            scenes: HashMap::new(),
            cur_scene:entry_scene,
            world: World::default(),
        }
    }

    pub fn frame_advance(&mut self, dt:f64){
        //println!(" frame_advancing");
    }
}
