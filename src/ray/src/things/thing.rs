
use crate::{color::Color, ray::Ray, surface::Surface, vector::Vector};

#[typetag::serde(tag = "type")]
pub trait Thing {
    fn intersect(&self, ray: &Ray) -> Vec<Vector>;
    fn surface(&self) -> &Surface;
    fn normal(&self, position: &Vector) -> Vector;

    //fn color_at(&self,position: Vec3dF32) -> Color;
}
