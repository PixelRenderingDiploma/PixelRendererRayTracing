use pixel_renderer_image_helper::common::Color;

use crate::common::{Intersection, Renderable};

pub trait LightSource {
    fn light_coefficient(&self, intersection: Intersection, objects: &Vec<Box<dyn Renderable>>) -> Color;
}

pub struct AmbientLightSource {
    pub color: Color,
    pub intensity: f32,

    ray_count: usize
}

impl LightSource for AmbientLightSource {
    fn light_coefficient(&self, intersection: Intersection, objects: &Vec<Box<dyn Renderable>>) -> Color {
        let mut light_coefficient = 0.0;

        // for i in 0..self.ray_count {
        //     let rand_ray = SampleLight();
        //     light_coefficient += ProcessDirection(randRay, intersection, sceneObjects);
        // }

        light_coefficient /= self.ray_count as f32;

        // return Color.FromShadowedColor(light_coefficient, self.color);
        self.color * light_coefficient
    }
}