use crate::scene::Scene;
use std::collections::HashMap;

pub struct App {
    scenes: HashMap<String,Scene>,
    entry_scene_id: String,
}



impl App{
    pub fn new()->Self{
        return Self{
            scenes: HashMap::new(),
            entry_scene_id:String::from(""),
        }
    }
    pub fn register_scene(&mut self,id: String,scene:Scene)->&Self{
        if self.scenes.contains_key(&id){
            panic!("sceneId {:?} has been used",id);
        }
        self.scenes.insert(id,scene);
        return self;
    }
    fn entry_scene(&mut self, id: String)->&Self{
        self.entry_scene_id = id;
        return self;
    }
}
