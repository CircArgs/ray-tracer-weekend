use super::materials::*;
use super::ray::*;
use super::vec3::*;
use core::fmt::Debug;

pub trait Intersect: Debug {
    fn intersect<T: Normal>(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit<T>>;
}

pub trait Normal: Intersect {
    fn normal(&self, point: &Vec3) -> Ray;
    fn material<T: Material>(&self) -> T;
}

#[derive(Debug)]
pub struct Hit<'a, T: Normal> {
    pub point: Vec3,
    pub distance: f32,
    pub object: &'a T,
}

impl<'a, T: Material + Normal> Hit<'a, T> {
    pub fn new(point: &Vec3, distance: f32, object: &'a T) -> Self {
        Hit {
            point: point.clone(),
            distance,
            object,
        }
    }

    pub fn normal(&self) -> Ray {
        self.object.normal(&self.point)
    }

    pub fn collide(&self, ray: &Ray) -> Ray {
        self.object.material::<T>().collide(ray, self)
    }

    pub fn albedo(&self) -> &Vec3 {
        self.object.material::<T>().albedo()
    }
}

#[derive(Debug)]
pub struct Intersectables<'a, T: Intersect> {
    pub objects: Vec<&'a T>,
}

impl<'a, T: Intersect> Intersectables<'a, T> {
    pub fn new(objects: Vec<&'a T>) -> Self {
        Intersectables { objects }
    }

    pub fn push(&mut self, object: &'a T) {
        self.objects.push(object);
    }
}

impl<'a, T: Material + Normal> Intersect for Intersectables<'a, T> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit<T>> {
        let mut ret: Option<Hit<T>> = None;
        for i in &self.objects {
            let temp = (*i).intersect(ray, t_min, t_max);
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
pub struct Sphere<T: Material + Debug> {
    center: Vec3,
    radius: f32,
    material: T,
}

impl<T: Material + Debug> Sphere<T> {
    pub fn new(center: &Vec3, radius: f32, material: &T) -> Self {
        Sphere {
            center: center.clone(),
            radius,
            material: material.clone(),
        }
    }
    pub fn center(&self) -> &Vec3 {
        &self.center
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl<'a, T: Material + Debug> Intersect for Sphere<T> {
    fn intersect<S: 'a + Normal>(&'a self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit<S>> {
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
            //take closest intersection
            if t1.abs() < t2.abs() {
                t = t1;
            } else {
                t = t2;
            }
            t *= 0.5; //divide by 2 in denom. of quad. formula
            if t < t_min || t > t_max {
                return None;
            }
            Some(Hit::new(&ray.parameterization(t), t, self))
        }
    }
}

impl<T: Material + Debug> Normal for Sphere<T> {
    fn normal(&self, point: &Vec3) -> Ray {
        Ray::new(point, &(point - &self.center))
    }
}
