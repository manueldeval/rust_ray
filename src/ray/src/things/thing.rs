use crate::{
    color::Color,
    ray::Ray,
    surfaces::Surface,
    vector::{Vector2d, Vector3d},
};

#[typetag::serde(tag = "type")]
pub trait Thing {
    fn intersect(&self, ray: &Ray) -> Vec<Vector3d>;
    fn surface(&self) -> &Surface;
    fn normal(&self, position: &Vector3d) -> Vector3d;

    fn get_uv_mapping(&self, position: &Vector3d) -> Vector2d;

    fn ambiant(&self, position: &Vector3d) -> Color {
        self.surface().ambiant(&self.get_uv_mapping(position))
    }
    fn diffuse(&self, position: &Vector3d) -> Color {
        self.surface().diffuse(&self.get_uv_mapping(position))
    }
    fn specular(&self, position: &Vector3d) -> Color {
        self.surface().specular(&self.get_uv_mapping(position))
    }
    fn refraction(&self, position: &Vector3d) -> Color {
        self.surface().refraction(&self.get_uv_mapping(position))
    }

    fn refraction_ratio(&self) -> f64 {
        self.surface().refraction_ratio()
    }
}
