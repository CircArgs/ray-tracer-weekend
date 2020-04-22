use super::ray::*;
use super::vec3::*;
use core::fmt::Debug;

pub trait Intersect: Debug {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

pub trait Normal {
    fn normal(&self, point: &Vec3) -> Ray;
}

#[derive(Debug)]
pub struct Hit {
    pub point: Vec3,
    pub distance: f32,
}

impl Hit {
    pub fn new(point: &Vec3, distance: f32) -> Hit {
        Hit {
            point: point.clone(),
            distance,
        }
    }
}

#[derive(Debug)]
pub struct Intersectables<'a> {
    pub objects: Vec<&'a dyn Intersect>,
}

impl<'a> Intersectables<'a> {
    pub fn new(objects: Vec<&'a dyn Intersect>) -> Self {
        Intersectables { objects }
    }

    pub fn push(&mut self, object: &'a dyn Intersect) {
        self.objects.push(object);
    }
}

impl<'a> Intersect for Intersectables<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut ret: Option<Hit> = None;
        for i in &self.objects {
            let temp = i.intersect(ray);
            //need to check if temp exists and if so if it is closer than we currently have
            match &temp {
                //if a hit
                Some(temp_hit) => {
                    //check if ret is None
                    match (&ret).as_ref() {
                        //if we already had a hit
                        Some(ret_hit) => {
                            //if our existing hit is further than our latest then our latest is our new existing hit
                            if ret_hit.distance > temp_hit.distance {
                                ret = temp;
                            }
                        }
                        //ret is None then we set temp to our new hit
                        None => {
                            ret = temp;
                        }
                    }
                }
                //no hit then don't do anything
                None => {}
            }
        }
        ret
    }
}

#[derive(Debug)]
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
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let a = 1.0;
        let b = 2.0 * ray.direction().dot(&(ray.origin() - &(self.center)));
        let c = ray.origin().squared_length() - 2.0 * ray.origin().dot(&(self.center))
            + self.center.squared_length()
            - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 1e-5 {
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
            t *= 0.5;

            Some(Hit::new(&ray.parameterization(t), t))
        }
    }
}

impl Normal for Sphere {
    fn normal(&self, point: &Vec3) -> Ray {
        Ray::new(point, &(point - &self.center))
    }
}
