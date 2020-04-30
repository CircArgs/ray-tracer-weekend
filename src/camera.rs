use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov: f32,
        aspect: f32,
        lens_radius: f32,
    ) -> Camera {
        let origin = lookfrom;
        let mut w = lookfrom - lookat;
        let focus_distance = w.length();
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan() * focus_distance;
        let half_width = aspect * half_height;
        w = w.normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);
        let lower_left_corner =
            &(&(origin - &(&u * half_width)) - &(&v * half_height)) - &(&w * focus_distance);
        let horizontal = &u * (half_width * 2.0);
        let vertical = &v * (half_height * 2.0);
        Camera {
            origin: origin.clone(),
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: f32 = rand::thread_rng().gen_range(0.0, 2.0 * PI);
        let (x, y) = (u * rd.cos(), v * rd.sin());

        let mut offset = &(&self.u * x) + &(&self.v * y);
        offset = &offset * self.lens_radius;
        let source = &self.origin + &offset;
        Ray::new(
            &source,
            &(&(&(&self.lower_left_corner + &(&self.horizontal * u)) + &(&self.vertical * v))
                - &source),
        )
    }
}
