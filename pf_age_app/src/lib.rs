pub mod app;
pub mod scene;

pub use app::App;
pub use pf_age_third_party::log::info;

#[cfg(target_os="android")]
pub mod android;
#[cfg(target_os="android")]
pub mod events;

pub mod render;
