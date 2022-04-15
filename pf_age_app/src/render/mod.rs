
mod render;
mod opengl;
pub mod fyrox_render;

pub use render::RenderTrait;
#[cfg(target_os="android")]
pub use opengl::GLRender;


