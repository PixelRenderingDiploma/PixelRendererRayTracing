use crate::vectors::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub direction: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>) -> Self {
        Ray { origin, direction }
    }

    pub fn from_to(from: Vec3<f32>, to: Vec3<f32>) -> Self {
        Ray { origin: from, direction: to - from }
    }
}