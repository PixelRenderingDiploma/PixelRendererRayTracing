use crate::vectors;

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub distance: f32,
    pub position: vectors::Vec3<f32>
}