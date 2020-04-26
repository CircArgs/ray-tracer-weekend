mod camera;
mod materials;
mod ray;
mod shapes;
mod vec3;
use camera::*;
use materials::*;
use rand::Rng;
use ray::*;
use shapes::*;
use std::f32::{consts, INFINITY};
use std::fs::File;
use std::io::Write;
use vec3::*;

fn color(ray: &Ray, world: &Intersectables, max_hits: i32) -> Vec3 {
    if max_hits == 0 {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return &(&Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Vec3::new(0.5, 0.7, 1.0) * t);
    }
    match world.intersect(ray, 0.001, INFINITY) {
        Some(hit) => {
            return &color(
                &Ray::from_spherical(
                    &hit.point,
                    rand::thread_rng().gen_range(0.0, consts::PI),
                    rand::thread_rng().gen_range(0.0, 2.0 * consts::PI),
                ),
                world,
                max_hits - 1,
            ) * 0.5;
        }
        _ => {
            let unit_direction = ray.direction().normalize();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return &(&Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Vec3::new(0.5, 0.7, 1.0) * t);
        }
    };
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let mut data = format!("P3\n{} {} \n255\n", nx, ny);
    let material1 = Lambertian::new(&Vec3::new(0.8, 0.3, 0.3));
    let material2 = Lambertian::new(&Vec3::new(0.8, 0.8, 0.0));
    let material3 = Lambertian::new(&Vec3::new(0.8, 0.6, 0.2));
    let material4 = Lambertian::new(&Vec3::new(0.8, 0.8, 0.8));
    let sphere1 = Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5, &material1);
    let sphere2 = Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0, &material2);
    let world = Intersectables::new(vec![&sphere1, &sphere2]);
    let camera = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let rand: f32 = rand::thread_rng().gen_range(0.0, 1.0);
                let u = ((i as f32) + rand) / (nx as f32);
                let v = ((j as f32) + rand) / (ny as f32);
                let r = camera.get_ray(u, v);
                col += &color(&r, &world, 10000);
            }
            col /= ns as f32;
            let ir = col.r();
            let ig = col.g();
            let ib = col.b();
            data.push_str(&format!(
                "{} {} {}\n",
                ((255.99 * ir) as i32),
                ((255.99 * ig) as i32),
                ((255.99 * ib) as i32)
            ));
        }
    }
    let mut f = File::create("test.ppm").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}
