use crate::common::*;
use crate::vectors::{Vec2, Vec3};

pub trait Scene {
    fn main_camera(&self) -> &Camera;
    fn objects(&self) -> &Vec<Box<dyn Renderable>>;
    fn light_sources(&self) -> &Vec<Box<dyn LightSource>>;

    fn add(&mut self, renderable: Box<dyn Renderable>);
}

// BuildScene
pub struct BuildScene {
    renderables: Vec<Box<dyn Renderable>>,
    light_sources: Vec<Box<dyn LightSource>>,
    camera: Camera
}

impl BuildScene {
    pub fn new() -> Self {
        BuildScene {
            renderables: vec![],
            light_sources: vec![],
            camera: Camera {
                transform: Transform::from_position(Vec3::new(0.0, 0.0, -5.0)),
                aspect: 1.0,
                fov: 60.0,
                near: 0.0,
                far: 0.0,
            }
        }
    }
}

impl Scene for BuildScene {
    fn main_camera(&self) -> &Camera {
        &self.camera
    }

    fn objects(&self) -> &Vec<Box<dyn Renderable>> {
        &self.renderables
    }
    
    fn light_sources(&self) -> &Vec<Box<dyn LightSource>> {
        &self.light_sources
    }

    fn add(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }
}

// FileScene
pub struct FileScene {
    renderables: Vec<Box<dyn Renderable>>,
}

impl FileScene {
    pub fn new() -> Self {
        FileScene { renderables: vec![] }
    }
}

impl Scene for FileScene {
    fn main_camera(&self) -> &Camera {
        unimplemented!()
    }

    fn objects(&self) -> &Vec<Box<dyn Renderable>> {
        unimplemented!()
    }

    fn light_sources(&self) -> &Vec<Box<dyn LightSource>> {
        unimplemented!()
    }

    fn add(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }
}
