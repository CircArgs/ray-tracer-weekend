use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(lookfrom: &Vec3, lookat: &Vec3, vup: &Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);
        let lower_left_corner = &(&(origin - &(&u * half_width)) - &(&v * half_height)) - &w;
        let horizontal = &u * (half_width * 2.0);
        let vertical = &v * (half_height * 2.0);
        Camera {
            origin: origin.clone(),
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            &self.origin,
            &(&(&(&self.lower_left_corner + &(&self.horizontal * u)) + &(&self.vertical * v))
                - &self.origin),
        )
    }
}
