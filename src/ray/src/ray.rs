use crate::vector::Vector;


#[derive(Debug)]
pub struct Ray {
    start: Vector,
    dir: Vector,
}

impl Ray {
    pub fn new(start: &Vector, dir: &Vector) -> Self {
        let start = start.clone();
        let dir= dir.norm().unwrap();
        Self {
            start,
            dir,
        }
    }  

    pub fn start(&self) -> &Vector {
        &self.start
    }

    pub fn dir(&self) -> &Vector {
        &self.dir
    }

}
