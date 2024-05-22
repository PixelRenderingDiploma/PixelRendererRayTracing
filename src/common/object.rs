use crate::common::*;

pub struct Object {
    pub transform: Transform,
    pub mesh: Mesh,
}

impl Renderable for Object {
    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;

        for triangle in self.mesh.triangles.iter() {
            intersection = triangle.intersect(ray, t_min, t_max);
            if intersection.is_some() { break; }
        }

        return intersection
    }
}