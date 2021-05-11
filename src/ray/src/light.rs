
use crate::{color::Color, vector::Vector3d};

#[derive(Serialize,Deserialize)]
pub struct Light {
    position: Vector3d,
    color: Color,
}

impl Light {
    pub fn new(position: Vector3d, color: Color) -> Self {
        Self { position, color }
    }

    pub fn position(&self) -> &Vector3d {
        &self.position
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}
