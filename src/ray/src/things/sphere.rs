use crate::{color::Color, ray::Ray, surface::Surface, vector::Vector};

use super::Thing;

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    radius: f64,
    position: Vector,
    surface: Surface,
}

impl Sphere {
    pub fn new(position: Vector, radius: f64, surface: Surface) -> Self {
        Self { position, radius , surface}
    }
}

#[typetag::serde(name = "sphere")]
impl Thing for Sphere {

    fn normal(&self, position: &Vector) -> Vector {
        (position.clone() - self.position.clone()).norm().unwrap()
    }

        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
    fn intersect(&self, ray: &Ray) -> Vec<Vector> {
        // let l: Vector = self.position.clone() - ray.start().clone();
        // let adj = l.dot(&ray.dir());
        // let d2 = l.dot(&l) - (adj * adj);
        // let radius2 = self.radius * self.radius;
        // if d2 > radius2 {
        //     return None;
        // }
        // let thc = (radius2 - d2).sqrt();
        // let t0 = adj - thc;
        // let t1 = adj + thc;
 
        // if t0 < 0.0 && t1 < 0.0 {
        //     return None;
        // }
        
        // let distance =  match (t0.is_sign_positive(),t1.is_sign_positive()) {
        //     (false,false) => None,      // Ray in opposite direction
        //     (false,true) => Some(t1),   // Inside the cirle
        //     (true,false) => Some(t0),   // Inside the cirle
        //     (true,true)  => Some(t0.min(t1)), // Outside the cirle
        // };

        // distance.map(|t| &(ray.dir() * &t.into()) + ray.start())


        let l: Vector = self.position.clone() - ray.start().clone();
        let adj = l.dot(&ray.dir());
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return vec!();
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;
 
        vec!(&(ray.dir() * &t0.into()) + ray.start(),
        &(ray.dir() * &t1.into()) + ray.start())
    }

    fn surface(&self) -> &Surface {
        &self.surface
    }

}
