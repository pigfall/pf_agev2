use bevy::prelude::{*};
#[pf_age_macro::pf_age_bevy()]
fn app_main(){
    App::new().set_runner(|e|loop{println!("looping2")}).run();
}
