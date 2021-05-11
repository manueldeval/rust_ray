use std::cmp::Ordering;


use crate::{color::{Color, BLACK, WHITE}, image::Image, intersection::{EPSILON, Intersection}, light::Light, ray::Ray, world::World};

pub struct Engine {
    world: World,
}

impl Engine {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub fn generate(&self) -> Image {
        let (width, height) = self.world.camera().get_pixel_size();
        let mut image = Image::new(width, height, WHITE);

        for y in 0..height {
            for x in 0..width {
                let color = self.get_pixel_at(x, y);
                image.set_color(x, y, color);
            }
        }
        image
    }

    fn get_pixel_at(&self, x: u16, y: u16) -> Color {
        let ray = self.world.camera().get_ray(x, y);
        self.launch_ray(&ray)
    }

    fn launch_ray(&self, ray: &Ray) -> Color {
        match self.find_intersection(&ray) {
            Some(inter) => {
                self.ambiant_component(&inter)
                    + self.diffuse_component(&inter)
                    + self.specular_component(&inter)
            }
            None => BLACK,
        }
    }

    fn ambiant_component(&self, intersection: &Intersection) -> Color {
        &self.world
            .thing(intersection.thing_index())
            .ambiant(intersection.position())
            * self.world.ambiant_light()
    }

    fn diffuse_component(&self, intersection: &Intersection) -> Color {
        self.world
            .lights()
            .iter()
            .filter_map(|light| self.diffuse_component_from_one_light(intersection, light))
            .sum()
    }

    fn diffuse_component_from_one_light(
        &self,
        intersection: &Intersection,
        light: &Light,
    ) -> Option<Color> {
        let intersection_to_light = light.position() - intersection.position();
        let distance_to_light = intersection_to_light.mag();
        let ray_to_light = Ray::new(intersection.position(), &intersection_to_light);
        let thing_normal = intersection.normal();
        let diffusion_coef = match (ray_to_light.dir().norm().unwrap()).dot(&thing_normal) {
            d if d > f64::EPSILON => d,
            _ => 0.0,
        };

        if diffusion_coef > 0.0 {
            match self.find_intersection(&ray_to_light) {
                Some(obstruction) if obstruction.distance() < distance_to_light => None,
                _ => Some(
                    (light.color()
                        * &self
                            .world
                            .thing(intersection.thing_index())
                            .diffuse(intersection.position()))
                        .scale(diffusion_coef),
                ),
            }
        } else {
            None
        }
    }

    fn specular_component(&self, _intersection: &Intersection) -> Color {
        BLACK
    }

    fn find_intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.world
            .things()
            .iter()
            .enumerate()
            // Find intersections with thing
            .filter_map(|(index, thing)| {
                thing
                    .intersect(ray)
                    .map(|intersection| (index, intersection.clone()))
            })
            // Create intersection object
            .map(|(thing_index, position)| {
                let mut normal = self.world.things()[thing_index].normal(&position);
                if normal.dot(ray.dir()) > 0.0 {
                    normal = - normal;
                }
                Intersection::new(
                    thing_index,
                    position.clone(),
                    normal,
                    (ray.start() - &position).mag(),
                )
            })
            .filter(|intersection| intersection.distance() > EPSILON)
            .min_by(|a, b| {
                if a.distance() < b.distance() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
    }
}
