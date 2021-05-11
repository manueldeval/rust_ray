use crate::vector::Vector;


#[derive(Debug)]
pub struct Ray {
    start: Vector,
    dir: Vector,
}

impl Ray {
    pub fn new(start: &Vector, dir: &Vector) -> Self {
        Self {
            start: start.clone(),
            dir: dir.norm().unwrap()
        }
    }  

    pub fn start(&self) -> &Vector {
        &self.start
    }

    pub fn dir(&self) -> &Vector {
        &self.dir
    }
}
