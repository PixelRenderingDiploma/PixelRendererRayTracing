mod vectors;
mod common;

use pixel_renderer_image_helper::common as image_common;
use image_common::Writer;

use common::*;
use vectors::*;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let scene_path = &args[1];
    // let mut gltf_reader = GLTFReader::new(scene_path);
    
    let object = Object {
        transform: Transform::new(),
        mesh: Mesh::from_primitive(Triangle {
            vertices: [
                Vertex { position: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(0.0, 0.0) },
                Vertex { position: Vec3::new(0.0, 1.0, 0.0), normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(0.0, 0.5) },
                Vertex { position: Vec3::new(1.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(0.5, 0.0) },
            ]
        })
    };
    
    let mut scene = Box::new(BuildScene::new());
    scene.add(Box::new(object));
    scene.add(Box::new(Sphere { transform: Transform::default(), radius: 0.5 }));

    let renderer = RayTracingRenderer { size: Vec2 { x: 32, y: 32 } };
    let image = renderer.render(scene);

    // let image = image_common::Image::from_mat(8, 8, vec![vec![image_common::Color::from_rgb(100, 0, 255); 8]; 8]);
    let writer = pixel_renderer_image_helper::ppm_writer::PPMWriter {};
    writer.write(image, "output/image.ppm");
}