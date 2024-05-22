use num::complex::ComplexFloat;

use crate::common::*;

pub struct Triangle {
    pub vertices: [Vertex; 3]
}

impl Primitive for Triangle {
    fn vertices(&self) -> Vec<Vertex> {
        self.vertices.to_vec()
    }
}

impl Renderable for Triangle {
    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let epsilon = 0.000001f32;

        //find vector for two edges sharing vertex1
        let edge1 = self.vertices[1].position - self.vertices[0].position;
        let edge2 = self.vertices[2].position - self.vertices[0].position;

        //begin calculating determinant - also used to calculate u-parameter
        let pvec = ray.direction.cross(edge2);

        //if determinant is near zero, ray lies in plane of triangle 
        let det = edge1.dot(pvec);
        if det > -epsilon && det < epsilon { return None; }

        let inv_det = 1.0 / det;

        //calculate distance from vertex1 to ray origin 
        let tvec = ray.origin - self.vertices[0].position;

        // calculate u-parameter and test bounds 
        let u = inv_det * tvec.dot(pvec);
        if u < 0.0 || u > 1.0 { return None; }

        //prepare to test v-parameter
        let qvec = tvec.cross(edge1);

        // calculate v-parameter and test bounds
        let v = inv_det * ray.direction.dot(qvec);
        if v < 0.0 || u + v > 1.0 { return None; }

        // calculate t, ray intersects triangle 
        let distance = inv_det * edge2.dot(qvec);
        if distance <= epsilon { return None; }

        let intersection_point = ray.origin + ray.direction * distance;
        Some(Intersection { distance, position: intersection_point })
    }
}