use crate::{color::Color, vector::Vector};

#[typetag::serde(tag = "type")]
pub trait ColorAt {
    fn color(&self, position: &Vector) -> Color;
}

#[derive(Serialize, Deserialize)]
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
    specular: Box<dyn ColorAt>,
    refraction: Box<dyn ColorAt>,
    refraction_ratio: f64,
}

impl Surface {
    pub fn new(
        ambiant: Box<dyn ColorAt>,
        diffuse: Box<dyn ColorAt>,
        specular: Box<dyn ColorAt>,
        refraction: Box<dyn ColorAt>,
        refraction_ratio: f64,
    ) -> Self {
        Self {
            ambiant,
            diffuse,
            specular,
            refraction,
            refraction_ratio,
        }
    }

    pub fn ambiant(&self, position: &Vector) -> Color {
        self.ambiant.color(position)
    }

    pub fn diffuse(&self, position: &Vector) -> Color {
        self.diffuse.color(position)
    }

    pub fn specular(&self, position: &Vector) -> Color {
        self.specular.color(position)
    }

    pub fn refraction(&self, position: &Vector) -> Color {
        self.refraction.color(position)
    }
    pub fn refraction_ratio(&self) -> f64 {
        self.refraction_ratio
    }
    
}
