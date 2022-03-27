use std::any::Any;

use crate::render::RenderTrait;
pub struct Scene<Render:RenderTrait> {
    pub one_frame_fn: fn(data: &mut Box<dyn Any>,dt:f64),
    pub data: Box<dyn Any>,
    pub render_fn: fn(data:&mut Box<dyn Any>,render :&mut Render),
}


impl <Render:RenderTrait> Scene<Render>{
    pub fn one_frame(&mut self, dt:f64){
        (self.one_frame_fn)(&mut self.data,dt);
    }
    pub fn render(&mut self,render:&mut Render){
        (self.render_fn)(&mut self.data,render);
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