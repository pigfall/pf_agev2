

#[cfg(target_os="android")]
pub mod opengl;
#[cfg(target_os="android")]
pub use opengl::GLRender;

