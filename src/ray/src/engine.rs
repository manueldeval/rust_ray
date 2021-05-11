use std::{cmp::Ordering, collections::btree_map::Range};

use crate::{
    color::{Color, BLACK, WHITE},
    image::Image,
    intersection::{Intersection, EPSILON},
    light::Light,
    ray::Ray,
    things::Thing,
    vector::Vector,
    world::World,
};

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
        self.launch_ray(&ray, self.world.max_recurions())
    }

    fn launch_ray(&self, ray: &Ray, max_recurions: u16) -> Color {
        match self.find_intersection(&ray) {
            Some(inter) => {
                let thing = self.world.thing(inter.thing_index());

                self.ambiant_component(&inter, thing)
                    + self.diffuse_component(&inter, thing)
                    + self.specular_component(&inter, thing, ray, max_recurions)
                    + self.refraction_component(&inter, thing, &ray, max_recurions)
            }
            None => BLACK,
        }
    }

    fn ambiant_component(&self, intersection: &Intersection, thing: &Box<dyn Thing>) -> Color {
        &thing.surface().ambiant(intersection.position()) * self.world.ambiant_light()
    }

    fn diffuse_component(&self, intersection: &Intersection, thing: &Box<dyn Thing>) -> Color {
        self.world
            .lights()
            .iter()
            .filter_map(|light| self.diffuse_component_from_one_light(intersection, thing, light))
            .sum()
    }

    fn diffuse_component_from_one_light(
        &self,
        intersection: &Intersection,
        thing: &Box<dyn Thing>,
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
                    (light.color() * &thing.surface().diffuse(intersection.position())).scale(diffusion_coef),
                ),
            }
        } else {
            None
        }
    }

    fn specular_component(
        &self,
        intersection: &Intersection,
        thing: &Box<dyn Thing>,
        ray: &Ray,
        max_recurions: u16,
    ) -> Color {
        if max_recurions == 0 || thing.surface().specular(intersection.position()).is_black() {
            BLACK
        } else {
            let new_ray_dir = Self::find_specular_direction(ray.dir(), intersection.normal());
            let new_ray = Ray::new(intersection.position(), &new_ray_dir);
            self.launch_ray(&new_ray, max_recurions - 1) * thing.surface().specular(intersection.position())
        }
    }

    fn find_specular_direction(ray_dir: &Vector, normal: &Vector) -> Vector {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel
        ray_dir - &(normal * &(2.0 * ray_dir.dot(normal)).into())
    }

    fn refraction_component(
        &self,
        intersection: &Intersection,
        thing: &Box<dyn Thing>,
        ray: &Ray,
        max_recurions: u16,
    ) -> Color {
        if max_recurions == 0 || thing.surface().refraction(intersection.position()).is_black() {
            BLACK
        } else {
            let material_refraction_index = thing.surface().refraction_ratio();
            let ratio = if intersection.collide_from_outside() {
                material_refraction_index
            } else {
                1.0 / material_refraction_index
            };

            let c = -intersection.normal().dot(ray.dir());

            let refraction_vector = ray.dir() * &ratio.into()
                + intersection.normal()
                    * &(ratio * c - (1.0 - (ratio * ratio) * (1.0 - (c * c))).sqrt()).into();

            let new_ray = Ray::new(intersection.position(), &refraction_vector);

            self.launch_ray(&new_ray, max_recurions - 1) * thing.surface().refraction(intersection.position())
        }
    }

    fn find_intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.world
            .things()
            .iter()
            .enumerate()
            // Find intersections with thing
            .flat_map(|(index, thing)| {
                let intersections = thing.intersect(ray);
                let intersections_with_index: Vec<(usize, Vector)> = intersections
                    .iter()
                    .map(|intersection| (index.clone(), intersection.clone()))
                    .collect();
                intersections_with_index.into_iter()
            })
            // Create intersection object
            .map(|(thing_index, position)| {
                let normal = self.world.thing(thing_index).normal(&position);
                let (normal, collide_from_outside) = if normal.dot(ray.dir()) > 0.0 {
                    (-normal, false)
                } else {
                    (normal, true)
                };
                Intersection::new(
                    thing_index,
                    position.clone(),
                    normal,
                    (ray.start() - &position).mag(),
                    collide_from_outside,
                )
            })
            // Avoid self intersection
            .filter(|intersection| intersection.distance() > EPSILON)
            .filter(|intersection| (intersection.position() - ray.start()).dot(ray.dir()) > 0.0)
            .min_by(|a, b| {
                if a.distance() < b.distance() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
    }
}
