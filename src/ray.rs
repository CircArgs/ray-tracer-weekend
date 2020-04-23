use super::vec3::*;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction.normalize(),
        }
    }
    pub fn parameterization(&self, t: f64) -> Vec3 {
        &(&self.direction * t) + &self.origin
    }
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
}
