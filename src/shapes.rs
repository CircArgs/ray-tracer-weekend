use super::materials::*;
use super::ray::*;
use super::vec3::*;
use core::fmt::Debug;

pub trait Intersect: Debug {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub trait Normal: Intersect {
    fn normal(&self, point: &Vec3) -> Ray;
    fn material(&self) -> &dyn Material;
}

#[derive(Debug)]
pub struct Hit<'a> {
    pub point: Vec3,
    pub distance: f32,
    pub object: &'a dyn Normal,
}

impl<'a> Hit<'a> {
    pub fn new(point: &Vec3, distance: f32, object: &'a dyn Normal) -> Self {
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
        self.object.material().collide(ray, self)
    }

    pub fn albedo(&self) -> &Vec3 {
        self.object.material().albedo()
    }
}

#[derive(Debug)]
pub struct Intersectables<'a> {
    pub objects: Vec<&'a dyn Normal>,
}

impl<'a> Intersectables<'a> {
    pub fn new(objects: Vec<&'a dyn Normal>) -> Self {
        Intersectables { objects }
    }

    pub fn push(&mut self, object: &'a dyn Normal) {
        self.objects.push(object);
    }
}

impl<'a> Intersect for Intersectables<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut ret: Option<Hit> = None;
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
pub struct Sphere<'a> {
    center: Vec3,
    radius: f32,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: &Vec3, radius: f32, material: &'a dyn Material) -> Self {
        Sphere {
            center: center.clone(),
            radius,
            material,
        }
    }
    pub fn center(&self) -> &Vec3 {
        &self.center
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl<'a> Intersect for Sphere<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        println!("sphere ray {:?}", ray);
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
            let mut t1 = -b + t;
            let mut t2 = -b - t;
            //divide by 2 in denom. of quad. formula
            t1 *= 0.5;
            t2 *= 0.5;
            println!("{} {}", t1, t2);
            //take closest intersection
            if t1.abs() > t2.abs() {
                let temp = t1;
                t1 = t2;
                t2 = temp;
            }

            if t1 > t_min && t1 < t_max {
                t = t1;
            } else if t2 > t_min || t2 < t_max {
                t = t2
            } else {
                return None;
            }
            println!("{} ", t);
            Some(Hit::new(&ray.parameterization(t), t, self))
        }
    }
}

impl<'a> Normal for Sphere<'a> {
    fn normal(&self, point: &Vec3) -> Ray {
        Ray::new(point, &(point - &self.center))
    }
    fn material(&self) -> &dyn Material {
        self.material
    }
}
