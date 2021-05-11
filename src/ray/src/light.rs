
use crate::{color::Color, vector::Vector};

#[derive(Serialize,Deserialize)]
pub struct Light {
    position: Vector,
    color: Color,
}

impl Light {
    pub fn new(position: Vector, color: Color) -> Self {
        Self { position, color }
    }

    pub fn position(&self) -> &Vector {
        &self.position
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}
