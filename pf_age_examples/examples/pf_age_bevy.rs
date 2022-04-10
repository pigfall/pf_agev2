use pf_age_app::{self, render::GLRender, scene::Scene, components::{mesh::Mesh, RenderFn}};
use pf_age_third_party::{glow, glow::HasContext, log::info,legion};
use bevy::prelude::{*};
#[pf_age_macro::pf_age_bevy()]
fn app_main(){
    App::new().build().run();
}
