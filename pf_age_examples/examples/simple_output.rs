use pf_age_app::{self, render::GLRender, scene::Scene};
use pf_age_third_party::{glow, glow::HasContext, log::info};
#[pf_age_macro::pf_age_main()]
fn app_main() -> App<GLRender> {
    return pf_age_app::App::new(Scene {
        data: Box::new(1i32),
        one_frame_fn: |data, dt| {
            let d = data.downcast_mut::<i32>().unwrap();
            *d = *d + 1;
            //info!("app one frame {:?}",d);
        },
        render_fn: |data, render| {
            //info!("scene rendering");
            if let Some(gl_fns) = &mut render.gl_fn_wrapper {
                if let Some(egl_wrapper) = &mut render.egl_wrapper {
                    if let Some(surface) = egl_wrapper.surface {
                        unsafe {
                            gl_fns.gl_raw_fns.clear_color(0.1, 0.2, 0.3, 0.5);
                            gl_fns.gl_raw_fns.clear(glow::COLOR_BUFFER_BIT);
                        };
                        egl_wrapper.swap_buffer()
                    }
                }
            }
        },
    });
}
