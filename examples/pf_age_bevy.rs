use bevy::prelude::{*};
// use pf_age_app::android::bevy_plugin::AndroidNativeAcitivityPlugin;

#[pf_age_macro::pf_age_bevy()]
fn app_main(){
    //App::new().add_plugin(AndroidNativeAcitivityPlugin{}).run();
    App::new().set_runner(|a|{println!("running2")}).run();
}
