use super::buffer::VertexBuffer;
use crate::material::Material;
// has associated surface data and material to render surface data
pub struct Surface{
    data: SurfaceData,
    material: Material,
}

pub struct SurfaceData{
    vertex_buffer:VertexBuffer
}
