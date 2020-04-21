use super::ray::*;
use super::vec::*;

trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Vec3>;
}


pub struct Sphere{
    center: Vec3,
    radius: f32
}

impl Sphere{
    pub fn new(center: &Vec3, radius)-> Sphere{
        Sphere{center: center.clone(), radius}
    }
    pub fn center(&self)-> &Vec3{
        &self.center
    }
    pub fn radius(&self)-> f32{
        self.radius
    }

}

impl Intersect for Sphere{
    fn intersect(&self, ray: &Ray)-> Option<Vec3>{
        let a = 1.0;
        let b = 2.0 * ray.direction().dot(ray.origin()-&(self.center));
        let c = ray.origin().squared_length()-2*ray.origin().dot(&(self.center))+self.center.squared_length()-self.radius*self.radius;
        let discriminant = b*b-4.0*a*c;
        if(discriminant<0){
            None
        }else{
            let t=(-b+discriminant.sqrt())*0.5;
            Some(ray.parameterization(t))
        }
    }
}