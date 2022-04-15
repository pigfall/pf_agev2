use pf_age_app::{self, render::GLRender, scene::Scene};
use pf_age_third_party::{glow, glow::HasContext, log::info};
#[pf_age_macro::pf_age_main()]
fn app_main() -> App {
    return pf_age_app::App::new(Scene {});
}
