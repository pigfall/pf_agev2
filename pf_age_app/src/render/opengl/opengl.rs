use crate::scene::mesh::{Mesh};
pub struct Renderer{

}

impl Renderer{
    pub fn render_frame(&mut self,app:&App){
        for mesh in app.get_world.query_component_mesh(){

        }
    }
    fn draw_mesh(&mut self,mesh:&Mesh){
        todo!("");
    }
}
