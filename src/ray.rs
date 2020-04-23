use super::vec3::*;

#[derive(Debug)]
pub struct Ray {
    origin: Box<Vec3>,
    direction: Box<Vec3>,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: Box::new(origin.clone()),
            direction: Box::new(direction.normalize()),
        }
    }
    pub fn parameterization(&self, t: f32) -> Vec3 {
        &(&(*self.direction) * t) + &(*self.origin)
    }
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn from_spherical(origin: &Vec3, phi: f32, theta: f32) -> Self {
        //origin: source of ray
        //phi φ in [0, pi] indicates a deviation in radians from the +z axis
        //theta in [0, 2pi] indicates a deviation from the +x axis in the x-y plane
        Ray {
            origin: Box::new(origin.clone()),
            direction: Box::new(Vec3::from_spherical(1.0, phi, theta)),
        }
    }
}
