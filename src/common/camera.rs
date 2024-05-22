use crate::common::Transform;
use crate::vectors::Vec2;

pub struct Camera {
    pub transform: Transform,

    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}