use crate::common::*;
use crate::common::round;

pub struct Sphere {
    pub transform: Transform,
    pub radius: f32,
}

impl Renderable for Sphere {
    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let origin = ray.origin;
        let center = self.transform.position;
        let k = origin - center;

        let direction_squared = ray.direction.dot(ray.direction);
        let radius_squared = self.radius * self.radius;
        let k_squared = k.dot(k);

        let a = direction_squared;
        let b = 2.0 * ray.direction.dot(k);
        let c = k_squared - radius_squared;

        let discriminant_squared = b * b - 4.0 * a * c;
        if discriminant_squared < 0.0 { return None; }

        let x1 = (-b + discriminant_squared.sqrt()) / (2.0 * a);
        let x2 = (-b - discriminant_squared.sqrt()) / (2.0 * a);

        let t = if x1 > x2 && round(x2, 2) > 0.0 { x2 } else { x1 };
        
        if round(t, 2) <= 0.0 { return None; }

        Some(Intersection {
            distance: k.length(),
            position: ray.origin + (ray.direction * t)
        })
    }
}