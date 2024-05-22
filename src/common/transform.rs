use num::{Zero, One};

use crate::vectors::vec3::Vec3;

pub struct Transform {
    pub position: Vec3<f32>,
    pub direction: Vec3<f32>,
    pub up: Vec3<f32>,
    pub right: Vec3<f32>,
    pub scale: Vec3<f32>
}

impl Transform {    
    pub fn new() -> Self {
        Transform {
            position: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(-1.0, 0.0, 0.0),
            scale: Vec3::one()
        }
    }

    pub fn from_position(position: Vec3<f32>) -> Self {
        Transform {
            position: position,
            direction: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(-1.0, 0.0, 0.0),
            scale: Vec3::one()
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::new()
    }
}