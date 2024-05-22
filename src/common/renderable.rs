use crate::common::*;

pub trait Renderable {
    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
}