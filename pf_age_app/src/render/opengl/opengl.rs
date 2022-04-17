use crate::scene::mesh::{Mesh};
use crate::App;
use pf_age_third_party::legion::{Read,IntoQuery};
use crate::scene::mesh::{MeshComponent};
pub struct Renderer{

}

impl Renderer{
    pub fn entry_load()->Self{
        return Self{};
    }
    pub fn render_frame(&mut self,app:&App){
        let mut query = Read::<MeshComponent>::query();
        for mesh in query.iter(&app.world){
            println!("handle mesh");
        }
    }
    fn draw_mesh(&mut self,mesh:&Mesh){
        todo!("");
    }
}
