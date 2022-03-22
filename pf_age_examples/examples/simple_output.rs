use pf_age_app;
#[pf_age_macro::pf_age_main()]
fn app_main()->App{
    return pf_age_app::App::new();
}
