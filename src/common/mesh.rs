use crate::common::*;

pub struct Mesh {
    pub triangles: Vec<Triangle>
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {

        Mesh {
            triangles: Vec::new()
        }
    }

    pub fn from_primitive<T: Primitive>(primitive: T) -> Self {
        let vertces = primitive.vertices();
        let triangles: Vec<Triangle> = vertces.chunks(3).map(|chunk| {
            Triangle {
                vertices: [chunk[0], chunk[1], chunk[2]]
            }
        }).collect();

        Mesh { triangles }
    }
}