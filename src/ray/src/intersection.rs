use crate::vector::Vector;

pub const EPSILON: f64 = 0.000_000_001;
pub struct Intersection {
    position: Vector,
    thing_index: usize,
    distance: f64,
    normal: Vector,
}

impl Intersection {
    pub fn new(thing_index: usize, position: Vector, normal: Vector, distance: f64) -> Self {
        Self {
            position,
            thing_index,
            distance,
            normal,
        }
    }

    pub fn position(&self) -> &Vector {
        &self.position
    }

    pub fn normal(&self) -> &Vector {
        &self.normal
    }

    pub fn thing_index(&self) -> usize {
        self.thing_index
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}
