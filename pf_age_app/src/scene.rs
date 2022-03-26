use std::any::Any;
pub struct Scene{
    pub one_frame_fn: fn(data: &mut Box<dyn Any>,dt:f64),
    pub data: Box<dyn Any>,
}


impl Scene{
    pub fn one_frame(&mut self, dt:f64){
        (self.one_frame_fn)(&mut self.data,dt);
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