use crate::render::RenderTrait;
use pf_age_ndk::ANativeWindow;
use pf_age_third_party::egl;
use pf_age_third_party::glow;
use pf_age_third_party::log::info;
use std::os::raw::c_void;
use std::ptr::NonNull;

pub struct GLRender {
    egl_inited: bool,
    pub egl_wrapper: Option<EglWrapper>,
    pub gl_fn_wrapper: Option<GLFnsWrapper>,
}

pub struct GLFnsWrapper {
    pub gl_raw_fns: glow::Context
}
pub struct EglWrapper {
    pub egl_ins: egl::DynamicInstance<egl::EGL1_4>,
    pub display: egl::Display,
    pub config: egl::Config,
    pub ctx: egl::Context,
    pub surface: Option<egl::Surface>,
}


impl EglWrapper {
    pub fn swap_buffer(&mut self){
        if let Some(surface) = self.surface{
            self.egl_ins.swap_buffers(self.display, surface).unwrap();
        }
    }
    fn attach_ctx_to_surface(&self, surface: egl::Surface) {
        info!("⌛ Attach an EGL rendering context to EGL surfaces");
        let egl_ins = &self.egl_ins;
        egl_ins
            .make_current(self.display, Some(surface), Some(surface), Some(self.ctx))
            .map_err(|e| {
                info!("❌ Failed to attach egl rendering context to egl surfaces");
                e
            })
            .unwrap();
        info!("✅ Attached an EGL rendering context to EGL surfaces");
    }
    fn create_surface(&self, native_window: *const ANativeWindow) -> egl::Surface {
        let surface = unsafe {
            self.egl_ins
                .create_window_surface(
                    self.display,
                    self.config,
                    native_window as egl::NativeWindowType,
                    None,
                )
                .map_err(|e| {
                    info!("❌ Failed to create window surface {:?}", e);
                    e
                })
                .unwrap()
        };
        info!("✅  Created window surface");
        return surface;
    }
}

impl GLRender {
    pub fn new() -> GLRender {
        GLRender {
            egl_inited: false,
            egl_wrapper: None,
            gl_fn_wrapper: None,
        }
    }
}

impl GLRender {
    fn init_egl(&mut self) {
        info!("⌛ Loading egl functions");
        let egl_ins = unsafe {
            egl::DynamicInstance::<egl::EGL1_4>::load_required()
                .map_err(|e| {
                    info!("✅ Failed to load egl functions {:?}", e);
                    e
                })
                .unwrap()
        };
        info!("⌛ Getting default display");
        let display = egl_ins
            .get_display(egl::DEFAULT_DISPLAY)
            .ok_or_else(|| {
                let msg = "❌ noget default display";
                info!("{:?}", msg);
                msg
            })
            .unwrap();
        info!("✅ Getted default display");

        info!("⌛  Initing display");
        egl_ins
            .initialize(display)
            .map_err(|e| {
                info!("❌ Failed to init display {:?}", e);
                e
            })
            .unwrap();
        info!("✅ Inited display");

        info!("⌛ Choose config which matched the attributes we wanted");
        let attributes: Vec<egl::Int> = [
            egl::RED_SIZE,
            8,
            egl::GREEN_SIZE,
            8,
            egl::BLUE_SIZE,
            8,
            egl::NONE,
        ]
        .to_vec();
        let config: egl::Config = egl_ins
            .choose_first_config(display, &attributes)
            .map_err(|e| {
                info!("❌ Failed to choose first config {:?}", e);
                e
            })
            .unwrap()
            .ok_or_else(|| {
                let msg = "❌ No matched config ";
                info!("{:?}", msg);
                msg
            })
            .unwrap();
        info!("✅ Config choosed");
        // >>

        let context_attributes = [
            egl::CONTEXT_MAJOR_VERSION,
            2,
            egl::CONTEXT_MINOR_VERSION,
            0,
            egl::NONE,
        ];

        // << create_context
        info!("⌛ Creating context");
        let ctx = egl_ins
            .create_context(display, config, None, Some(&context_attributes))
            .map_err(|e| {
                info!("❌ Failed to create context {:?}", e);
                e
            })
            .unwrap();
        info!("✅ Created context");

        self.egl_wrapper = Some(EglWrapper {
            display: display,
            ctx: ctx,
            config: config,
            egl_ins: egl_ins,
            surface: None,
        });

        self.egl_inited = true;
    }
}

impl RenderTrait for GLRender {
    fn on_window_create(&mut self, window_ptr: *mut c_void) {
        if !self.egl_inited {
            self.init_egl();
        }
        info!("TODO opengl on_window_create");
        let egl_wrapper = self.egl_wrapper.as_mut().expect("inited egl");
        let surface = egl_wrapper.create_surface(window_ptr as _);
        egl_wrapper.attach_ctx_to_surface(surface);
        egl_wrapper.surface = Some(surface);

        if self.gl_fn_wrapper.is_none() {
            let gl_fns = unsafe {
                glow::Context::from_loader_function_with_version_parse(
                    |version| {
                        info!("gl version to parse {:?}", version);
                        Ok(glow::Version {
                            major: 1,
                            minor: 0,
                            is_embedded: true,
                            revision: None,
                            vendor_info: "tzz".to_string(),
                        })
                    },
                    |fn_name| {
                        info!("⏱️ loading gl fn {:?}",fn_name);
                        egl_wrapper.egl_ins.get_proc_address(fn_name).map_or(
                            std::ptr::null(),
                            |fn_ptr|{
                                fn_ptr as *const _
                            }
                        )
                    },
                ).expect("load gl fns failed")
            };
            self.gl_fn_wrapper = Some(GLFnsWrapper{gl_raw_fns:gl_fns});
            info!("❤️ gl fns loaded");
        }
    }
    fn on_window_destroy(&mut self, window_ptr: *mut c_void) {
        info!("TODO opengl on window destroy");
        if let Some(egl_wrapper) = &mut self.egl_wrapper{
            if let Some(surface) = egl_wrapper.surface{
                egl_wrapper.egl_ins.destroy_surface(egl_wrapper.display, surface).expect("failed to destroy surface");
                egl_wrapper.surface = None;
                info!("✅ destroy egl surface ");
            }
        }
    }
}
