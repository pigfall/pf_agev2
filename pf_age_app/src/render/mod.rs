
mod render;
mod opengl;

pub use render::RenderTrait;
#[cfg(target_os="android")]
pub use opengl::GLRender;
