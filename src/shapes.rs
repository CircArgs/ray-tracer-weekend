use super::ray::*;
use super::vec3::*;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Vec3>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center.clone(),
            radius,
        }
    }
    pub fn center(&self) -> &Vec3 {
        &self.center
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        let a = 1.0;
        let b = 2.0 * ray.direction().dot(&(ray.origin() - &(self.center)));
        let c = ray.origin().squared_length() - 2.0 * ray.origin().dot(&(self.center))
            + self.center.squared_length()
            - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let mut t: f32 = discriminant.sqrt();
            let t1 = -b + t;
            let t2 = -b - t;
            //neither pos no intersect in pos dir
            if (t1 < 1e-5) & (t2 < 1e-5) {
                return None;
            //both pos take closest
            } else if (t1 > 1e-5) & (t2 > 1e-5) {
                t = t1.min(t2);
            //one pos take pos
            } else {
                t = t1.max(t2);
            }

            Some(ray.parameterization(t * 0.5))
        }
    }
}
