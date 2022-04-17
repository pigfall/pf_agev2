
mod render;
mod opengl;

#[cfg(target_os="android")]
mod egl;
#[cfg(target_os="android")]
pub use egl::EglRenderer;

pub use render::Renderer;
