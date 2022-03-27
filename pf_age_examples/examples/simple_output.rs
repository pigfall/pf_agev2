use pf_age_app::{self, scene::Scene, render::GLRender};
use pf_age_third_party::log::info;
#[pf_age_macro::pf_age_main()]
fn app_main()->App<GLRender>{
    return pf_age_app::App::new(
        Scene{
            data:Box::new(1i32),
            one_frame_fn:|data,dt|{
                let d = data.downcast_mut::<i32>().unwrap();
                *d=*d+1;
                info!("app one frame {:?}",d);
            },
            render_fn:|data,render|{
                info!("scene rendering");
            },
        }
    );
}
