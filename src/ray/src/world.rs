use crate::{camera::Camera, color::Color, light::Light, things::Thing};

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]  
pub struct World {
    camera: Camera,
    things: Vec<Box<dyn Thing>>,
    lights: Vec<Light>,
    ambiant_light: Color,
}

impl World {
    pub fn new(camera: Camera, things: Vec<Box<dyn Thing>>, lights: Vec<Light>,ambiant_light: Color) -> Self {
        Self {
            camera,
            things,
            lights,
            ambiant_light
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn things(&self) -> &Vec<Box<dyn Thing>> {
        &self.things
    }

    pub fn thing(&self,index: usize) -> &Box<dyn Thing> {
        &self.things[index]
    }


    pub fn ambiant_light(&self) -> &Color {
        &self.ambiant_light
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }
}