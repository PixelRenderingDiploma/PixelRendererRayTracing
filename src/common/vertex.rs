use crate::vectors;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: vectors::Vec3<f32>,
    pub normal: vectors::Vec3<f32>,
    pub uv: vectors::Vec2<f32>,
}