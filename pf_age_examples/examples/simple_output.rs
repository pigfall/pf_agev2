use pf_age_app::{self, render::GLRender, scene::Scene, components::{mesh::Mesh, RenderFn}};
use pf_age_third_party::{glow, glow::HasContext, log::info,legion};
use pf_age_app::App;
#[pf_age_macro::pf_age_main()]
fn app_main() -> App {
    return pf_age_app::App::new(Scene::new(
    |world|{
        world.push(
            (
                RenderFn{
                    f:|gl_fns|{
                        unsafe{
                            gl_fns.clear_color(0.1,0.2,0.3,0.5);
                            gl_fns.clear(glow::COLOR_BUFFER_BIT);
                        };
                    }
                    },
                    Mesh{},
        ),
        
        );
    },
         legion::Schedule::builder().build(),
         legion::Resources::default(),
        ));
}
