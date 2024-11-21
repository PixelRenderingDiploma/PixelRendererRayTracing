mod vectors;
mod common;

use common::*;
use vectors::*;

use pixel_renderer_image_helper::{common as image_common, ppm_reader, ppm_writer};
use image_common::{Reader, Writer};

#[no_mangle]
pub extern fn read_write(source: *const i8, destination: *const i8) -> bool {
    println!("{:?}", source);
    println!("{:?}", destination);

    let src = unsafe { std::ffi::CStr::from_ptr(source).to_str() }.unwrap();
    let dst = unsafe { std::ffi::CStr::from_ptr(destination).to_str() }.unwrap();
    
    println!("{}", src);
    println!("{}", dst);

    let ppm_reader = ppm_reader::PPMReader {};
    let image = ppm_reader.read(src).unwrap();

    let writer = ppm_writer::PPMWriter {};
    writer.write(image, dst);

    return true
}

#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn render_model_ppm() {
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
}