use super::ray::*;
use super::shapes::*;
use super::vec3::*;
use rand::Rng;
use std::f32::consts;

pub trait Material: Clone {
    fn collide<T: Material>(&self, ray_in: &Ray, hit: &Hit<T>) -> Ray;
    fn albedo(&self) -> &Vec3;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn albedo(&self) -> &Vec3 {
        &self.albedo
    }
    pub fn new(albedo: &Vec3) -> Self {
        Lambertian {
            albedo: albedo.clone(),
        }
    }
}

impl<T: Material> Material for Lambertian {
    fn collide(&self, ray_in: &Ray, hit: &Hit<T>) -> Ray {
        Ray::from_spherical(
            &hit.point,
            rand::thread_rng().gen_range(0.0, consts::PI),
            rand::thread_rng().gen_range(0.0, 2.0 * consts::PI),
        )
    }
}
