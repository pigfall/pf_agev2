use pf_age_app::{self,  game_scene::Scene};
use pf_age_third_party::{glow, glow::HasContext, log::info};
use pf_age_app::scene::mesh::{MeshComponent};

#[pf_age_macro::pf_age_main()]
fn app_main() -> App {
    let mut app = pf_age_app::App::new(Scene {});
    app.world.push((MeshComponent{},Velocity{dx:0.0,dy:0.0}));
    return app
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    dx: f32,
    dy: f32,
}
