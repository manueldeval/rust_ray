use crate::{color::Color, vector::Vector2d};

#[typetag::serde(tag = "type")]
pub trait ColorAt {
    fn color(&self, uv: &Vector2d) -> Color;
}

#[derive(Serialize, Deserialize)]
pub struct ConstColor {
    color: Color,
}

#[typetag::serde(name = "const_color")]
impl ColorAt for ConstColor {
    fn color(&self, _uv: &Vector2d) -> Color {
        self.color.clone()
    }
}

impl ConstColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

// http://www.irisa.fr/prive/kadi/Cours_LR2V/RayTracing_Texturing.pdf
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

    pub fn ambiant(&self, uv: &Vector2d) -> Color {
        self.ambiant.color(uv)
    }

    pub fn diffuse(&self, uv: &Vector2d) -> Color {
        self.diffuse.color(uv)
    }

    pub fn specular(&self,uv: &Vector2d) -> Color {
        self.specular.color(uv)
    }

    pub fn refraction(&self, uv: &Vector2d) -> Color {
        self.refraction.color(uv)
    }
    pub fn refraction_ratio(&self) -> f64 {
        self.refraction_ratio
    }
    
}
