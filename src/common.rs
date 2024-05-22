mod renderable;
mod renderer;
mod scene;
mod camera;
mod transform;
mod ray;
mod mesh;
mod sphere;
mod object;
mod vertex;
mod triangle;
mod primitive;
mod intersection;
mod light_source;

pub use renderable::Renderable;
pub use renderer::*;
pub use transform::Transform;
pub use scene::*;
pub use ray::Ray;
pub use intersection::Intersection;
pub use mesh::Mesh;
pub use sphere::Sphere;
pub use object::Object;
pub use vertex::Vertex;
pub use triangle::Triangle;
pub use primitive::Primitive;
pub use light_source::LightSource;
pub use camera::Camera;

pub fn round(value: f32, decimals: i32) -> f32 {
    let factor = 10.0_f32.powi(decimals);
    (value * factor).round() / factor
}