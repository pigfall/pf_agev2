use std::any::Any;

use crate::render::RenderTrait;
pub struct Scene {
}


impl  Scene{
    pub fn update(&mut self,dt:f32){

    }
}

pub struct SceneBuilder{

}

impl SceneBuilder{
    pub fn new()->SceneBuilder{
        return SceneBuilder {  };
    }

    pub fn with_input_reader(&mut self)->&mut Self{

        return self;
    }
}
