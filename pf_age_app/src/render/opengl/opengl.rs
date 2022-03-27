use crate::render::RenderTrait;
use pf_age_third_party::egl;
use pf_age_third_party::log::info;
use std::os::raw::c_void;

pub struct GLRender {
    egl_inited: bool,
}

impl GLRender {
    pub fn new() -> GLRender {
        GLRender {egl_inited:false}
    }
}

impl GLRender {
    //fn init_egl(&mut self) {
    //    info!("⌛ Loading egl functions");
    //    let egl_ins = unsafe {
    //        egl::DynamicInstance::<egl::EGL1_4>::load_required()
    //            .map_err(|e| {
    //                info!("✅ Failed to load egl functions {:?}", e);
    //                e
    //            })
    //            .unwrap()
    //    };
    //    info!("⌛ Getting default display");
    //    let display = egl_ins
    //        .get_display(egl::DEFAULT_DISPLAY)
    //        .ok_or_else(|| {
    //            let msg = "❌ noget default display";
    //            info!("{:?}", msg);
    //            msg
    //        })
    //        .unwrap();
    //    info!("✅ Getted default display");

    //    info!("⌛  Initing display");
    //    egl_ins
    //        .initialize(display)
    //        .map_err(|e| {
    //            info!("❌ Failed to init display {:?}", e);
    //            e
    //        })
    //        .unwrap();
    //    info!("✅ Inited display");

    //    info!("⌛ Choose config which matched the attributes we wanted");
    //    let attributes: Vec<egl::Int> = [
    //        egl::RED_SIZE,
    //        8,
    //        egl::GREEN_SIZE,
    //        8,
    //        egl::BLUE_SIZE,
    //        8,
    //        egl::NONE,
    //    ]
    //    .to_vec();
    //    let config: egl::Config = egl_ins
    //        .choose_first_config(display, &attributes)
    //        .map_err(|e| {
    //            info!("❌ Failed to choose first config {:?}", e);
    //            e
    //        })
    //        .unwrap()
    //        .ok_or_else(|| {
    //            let msg = "❌ No matched config ";
    //            info!("{:?}", msg);
    //            msg
    //        })
    //        .unwrap();
    //    info!("✅ Config choosed");
    //    // >>

    //    let context_attributes = [
    //        egl::CONTEXT_MAJOR_VERSION,
    //        2,
    //        egl::CONTEXT_MINOR_VERSION,
    //        0,
    //        egl::NONE,
    //    ];

    //    // << create_context
    //    info!("⌛ Creating context");
    //    let ctx = egl_ins
    //        .create_context(display, config, None, Some(&context_attributes))
    //        .map_err(|e| {
    //            info!("❌ Failed to create context {:?}", e);
    //            e
    //        })
    //        .unwrap();
    //    info!("✅ Created context");

    //    let gl_ins = gl::GLIns {
    //        display: display,
    //        ctx: ctx,
    //        config: config,
    //        egl_ins: egl_ins,
    //        surface: None,
    //        gl_fcs: None,
    //    };
    //}
}

impl RenderTrait for GLRender {
    fn on_window_create(&mut self, window_ptr: *mut c_void) {
        info!("TODO opengl on_window_create");
    }
    fn on_window_destroy(&mut self, window_ptr: *mut c_void) {
        info!("TODO opengl on window destroy");
    }
}
