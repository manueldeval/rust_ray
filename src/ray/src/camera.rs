
use crate::{ray::Ray, vector::Vector};
#[derive(Debug,Deserialize,Serialize)]
pub struct Camera {
    direction: Vector,
    up: Vector,
    right: Vector,
    position: Vector,
    focal_dist: f64,
    image_pixels_width: u16,
    image_pixels_height: u16,
    pixel_per_unit: f64,
    image_len_width: f64,
    image_len_height: f64,
}

impl Default for Camera {
    fn default() -> Self {
        let direction = Vector::x_axis();
        let right = -Vector::y_axis();
        let up = Vector::z_axis();
        let position = Vector::zero();
        let focal_dist = 2.0;
        let image_pixels_width = 640;
        let image_pixels_height = 480;
        let pixel_per_unit = 320.0;
        let image_len_width = Self::compute_image_size(image_pixels_width, pixel_per_unit);
        let image_len_height = Self::compute_image_size(image_pixels_height, pixel_per_unit);
        Self {
            direction,
            right,
            up,
            position,
            focal_dist,
            image_pixels_width,
            image_pixels_height,
            pixel_per_unit,
            image_len_width,
            image_len_height,
        }
    }
}
impl Camera {
    pub fn debug(&self) {
        dbg!(&self.direction);
        dbg!(&self.right);
        dbg!(&self.up);
        dbg!(&self.position);

        dbg!(&self.focal_dist);

        dbg!(&self.image_pixels_width);
        dbg!(&self.image_pixels_height);
        dbg!(&self.pixel_per_unit);
        dbg!(&self.image_len_width);
        dbg!(&self.image_len_height);
        dbg!(self.image_spot());
        dbg!(self.up_left());

    }

    fn compute_image_size(pixels: u16, pixels_per_unit: f64) -> f64 {
        pixels as f64 / pixels_per_unit
    }

    pub fn get_pixel_size(&self) -> (u16, u16) {
        (self.image_pixels_width, self.image_pixels_height)
    }

    pub fn image_spot(&self) -> Vector {
        self.position.clone() - (self.direction.clone() * self.focal_dist.into())
    }

    pub fn up_left(&self) -> Vector {
        self.position.clone() 
            + (self.up.clone() * self.image_len_height.into() / 2.0.into())
            - (self.right.clone() * self.image_len_width.into() / 2.0.into())
    }

    pub fn get_ray(&self, pixel_x: u16, pixel_y: u16) -> Ray {
        let upleft_position = self.up_left();
        // Something bad there!
        let x_increment = self.right.clone() * (self.image_len_width * pixel_x as f64 / self.image_pixels_width as f64).into();
        let y_increment = -self.up.clone() * (self.image_len_height * pixel_y as f64 / self.image_pixels_height as f64).into();
        //dbg!(&x_increment);
        //dbg!(&y_increment);
        let point_on_screen = upleft_position + x_increment + y_increment;

        let dir = point_on_screen.clone() - self.image_spot();
        let start = self.image_spot().clone();
        Ray::new(&start, &dir)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_spot() {
        let camera = Camera::default();
        let image_spot = camera.image_spot();
        dbg!(image_spot);
        dbg!(camera);
    }

    #[test]
    fn test_ray() {
        let camera = Camera::default();
        dbg!(camera.get_ray(0, 0));
    }
}
