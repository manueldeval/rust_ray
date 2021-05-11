
use crate::{color::Color, vector::Vector};

#[typetag::serde(tag = "type")]
pub trait ColorAt {
    fn color(&self, position: &Vector) -> Color;
}

#[derive(Serialize,Deserialize)]
pub struct ConstColor {
    color: Color,
}

#[typetag::serde(name = "const_color")]
impl ColorAt for ConstColor {
    fn color(&self, _position: &Vector) -> Color {
        self.color.clone()
    }
}

impl ConstColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Surface {
    ambiant: Box<dyn ColorAt>,
    diffuse: Box<dyn ColorAt>,
}

impl Surface {
    pub fn new(ambiant: Box<dyn ColorAt>, diffuse: Box<dyn ColorAt>) -> Self {
        Self { ambiant, diffuse }
    }

    pub fn ambiant(&self, position: &Vector) -> Color {
        self.ambiant.color(position)
    }

    pub fn diffuse(&self, position: &Vector) -> Color {
        self.diffuse.color(position)
    }
}
