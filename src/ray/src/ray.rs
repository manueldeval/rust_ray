use crate::vector::Vector3d;


#[derive(Debug)]
pub struct Ray {
    start: Vector3d,
    dir: Vector3d,
}

impl Ray {
    pub fn new(start: &Vector3d, dir: &Vector3d) -> Self {
        let start = start.clone();
        let dir= dir.norm().unwrap();
        Self {
            start,
            dir,
        }
    }  

    pub fn start(&self) -> &Vector3d {
        &self.start
    }

    pub fn dir(&self) -> &Vector3d {
        &self.dir
    }

}
