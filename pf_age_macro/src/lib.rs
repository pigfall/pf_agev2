mod pf_age_main;

#[proc_macro_attribute]
pub fn pf_age_main(attr:proc_macro::TokenStream, input: proc_macro::TokenStream)->proc_macro::TokenStream{
    return pf_age_main::pf_age_main(attr,input);
}

#[proc_macro_attribute]
pub fn pf_age_bevy(attr:proc_macro::TokenStream, input: proc_macro::TokenStream)->proc_macro::TokenStream{
    return pf_age_main::pf_age_bevy(attr,input);
}
