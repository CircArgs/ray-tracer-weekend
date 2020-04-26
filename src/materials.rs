use super::ray::*;
use super::shapes::*;
use super::vec3::*;
use core::fmt::Debug;
use rand::Rng;
use std::f32::consts;

fn rand_in_unit_sphere() -> Vec3 {
    Vec3::from_spherical(
        1.0,
        rand::thread_rng().gen_range(0.0, consts::PI),
        rand::thread_rng().gen_range(0.0, 2.0 * consts::PI),
    )
}
pub trait Material: Debug {
    fn collide(&self, ray_in: &Ray, hit: &Hit) -> Ray;
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

impl Material for Lambertian {
    fn albedo(&self) -> &Vec3 {
        &self.albedo
    }
    fn collide(&self, ray_in: &Ray, hit: &Hit) -> Ray {
        Ray::from_spherical(
            &hit.point,
            rand::thread_rng().gen_range(0.0, consts::PI),
            rand::thread_rng().gen_range(0.0, 2.0 * consts::PI),
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn albedo(&self) -> &Vec3 {
        &self.albedo
    }
    pub fn new(albedo: &Vec3, fuzz: f32) -> Self {
        Metal {
            albedo: albedo.clone(),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn albedo(&self) -> &Vec3 {
        &self.albedo
    }
    fn collide(&self, ray_in: &Ray, hit: &Hit) -> Ray {
        let proj = hit.normal().direction() * ray_in.direction().dot(hit.normal().direction());
        let refl = ray_in.direction() - &(&proj * 2.0);
        Ray::new(&hit.point, &(&refl + &(&rand_in_unit_sphere() * self.fuzz)))
    }
}
