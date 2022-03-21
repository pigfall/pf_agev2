use proc_macro::{TokenStream};
use quote::{quote};

pub fn pf_age_main(attr: TokenStream,input: TokenStream)->TokenStream{

    let output_tks = quote!{

        use std::os::raw::c_void;

        #[cfg(target_os="windows")]
        compile_error!("unsupprot windows");

        #[cfg(target_os="android")]
        #[no_mangle]
        unsafe extern "C" fn ANativeActivity_onCreate(
            activity_raw_ptr: *mut c_void,
            saved_state_raw_ptr: *mut c_void,
            saved_state_size: usize,
            ){
            // { init logger
            //pf_age_entry::init_android_logger("pf_age_logger");
            //info!(" ANativeActivity_onCreating");
            // }
            let entry = #build_app();
            if let Some(engine) = entry.custome_game_entry{
                engine.android_activity_on_create();
            }else{
                engine::android_activity_on_create(entry.app);
            }
        }
    };

    return output_tks.into();
}
