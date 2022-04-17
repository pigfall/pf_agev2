pub mod surface;
use  surface::{Surface};

pub mod buffer;

// has many surface
pub struct Mesh{
    surfaces : Vec<Surface>,
}

