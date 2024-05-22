use std::time::Instant;

use pixel_renderer_image_helper::common::{Image, Color};

use crate::vectors::{Vec2, Vec3};
use crate::common::*;

pub trait Renderer {
    fn render(&self, scene: Box<dyn Scene>) -> Image;
}

pub struct RayTracingRenderer {
    pub size: Vec2<usize>,
}

impl Renderer for RayTracingRenderer {
    fn render(&self, scene: Box<dyn Scene>) -> Image {
        let mut image = Image::new(self.size.x, self.size.y);

        let vertical_scale = f32::tan(f32::to_radians(scene.main_camera().fov / 2.0));
        let aspect_ratio = scene.main_camera().aspect;
        let plane_height = 1.0 * vertical_scale * 2.0;
        let plane_width = plane_height * aspect_ratio;

        let step_down = plane_height / self.size.x as f32;
        let step_right = plane_width / self.size.y as f32;

        let upper_left_pixel_coords = Vec3::new(-(plane_width / 2.0) + step_right / 2.0, plane_height / 2.0 - step_down / 2.0, -3.0);

        let start = Instant::now();
        for i in 0..self.size.y {
            for j in 0..self.size.x {
                image.pixels[i][j] = Color::black();

                let ray = Ray::from_to(
                    scene.main_camera().transform.position,
                    upper_left_pixel_coords + Vec3::new(step_right * j as f32, -step_down * i as f32, 0.0)
                );

                let nearest_intersection = self.get_intersection_with_closest_object(ray, scene.objects());
                if nearest_intersection.is_none() { continue; }

                image.pixels[i][j] = Color::from_rgb(100, 100, 100);
                for light_source in scene.light_sources() {
                    image.pixels[i][j] += light_source.light_coefficient(nearest_intersection.unwrap(), scene.objects());
                }
            }
        }
        println!("{}", start.elapsed().as_secs());

        return image;
    }
}

impl RayTracingRenderer {
    fn get_intersection_with_closest_object(&self, ray: Ray, objects: &Vec<Box<dyn Renderable>>) -> Option<Intersection> {
        let mut hit_distance = f32::INFINITY;
        let mut closest_intersection = None::<Intersection>;
        
        for obj in objects {
            let intersection = obj.intersect(ray, 0.0, 1.0);

            if intersection.is_none() {
                continue;
            }

            let intersection_vector = intersection.unwrap().position - ray.origin;
            let distance = intersection_vector.length();

            if distance < hit_distance {
                hit_distance = distance;
                closest_intersection = intersection;
            }
        }

        return closest_intersection;
    }
}