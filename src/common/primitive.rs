use crate::common::*;

pub trait Primitive {
    fn vertices(&self) -> Vec<Vertex>;
}