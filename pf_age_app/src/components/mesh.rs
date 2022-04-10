use crate::render::{RenderTrait, GLRender};
use pf_age_third_party::glow;



pub struct Mesh{}


pub struct RenderFn{
    pub f: fn(&mut glow::Context)
}