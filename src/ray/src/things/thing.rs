
use crate::{color::Color, ray::Ray, vector::Vector};

#[typetag::serde(tag = "type")]
pub trait Thing {
    fn intersect(&self, ray: &Ray) -> Option<Vector>;
    fn ambiant(&self,position: &Vector) -> Color;
    fn diffuse(&self,position: &Vector) -> Color;
    fn normal(&self, position: &Vector) -> Vector;
    //fn color_at(&self,position: Vec3dF32) -> Color;
}
