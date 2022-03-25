use proc_macro::{TokenStream};
use quote::{quote};
use syn::{parse_macro_input, ItemFn};

pub fn pf_age_main(attr: TokenStream,input: TokenStream)->TokenStream{
    let item_ast = parse_macro_input!(input as ItemFn);
    let main_fn_name = &item_ast.sig.ident;

    let output_tks = quote!{
        use std::os::raw::c_void;
        use pf_age_app::App;

        #[cfg(target_os="windows")]
        compile_error!("unsupprot windows");

        #[cfg(target_os="android")]
        #[no_mangle]
        unsafe extern "C" fn ANativeActivity_onCreate(
            activity_raw_ptr: *mut c_void,
            saved_state_raw_ptr: *mut c_void,
            saved_state_size: usize,
            ){
            //pf_age_app::android::init_android_logger("pf_age");
            //pf_age_app::android::log::info!("activity on create");
            pf_age_app::android::android_platform_entry(activity_raw_ptr as *mut _,saved_state_raw_ptr,saved_state_size,#main_fn_name);
            //}
        }
        
        #item_ast
    };

    return output_tks.into();
}
