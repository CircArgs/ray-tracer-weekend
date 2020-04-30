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

fn color(ray: &Ray, world: &Intersectables, max_hits: u32) -> Vec3 {
    if max_hits == 0 {
        return Vec3::from_float(0.0);
    }
    match world.intersect(ray, 0.001, INFINITY) {
        Some(hit) => {
            return hit.albedo() * &color(&hit.collide(&ray), &world, max_hits - 1);
        }
        _ => {
            let t = 0.5 * (ray.direction().y() + 1.0);
            return &(&Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Vec3::new(0.5, 0.7, 1.0) * t);
        }
    };
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let mut data = format!("P3\n{} {} \n255\n", nx, ny);
    let material1 = Lambertian::new(&Vec3::new(0.8, 0.2, 0.5));
    let material2 = Lambertian::new(&Vec3::new(0.5, 0.8, 0.0));
    let material3 = Metal::new(&Vec3::new(0.5, 0.5, 0.5), 0.0);
    let material4 = Dielectric::new(1.5, 0.0);
    let sphere1 = Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5, &material1);
    let sphere2 = Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0, &material2);
    let sphere3 = Sphere::new(&Vec3::new(1.0, 0.0, -1.0), 0.5, &material3);
    let sphere4 = Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), 0.5, &material4);
    let sphere5 = Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), -0.49, &material4);
    let world = Intersectables::new(vec![&sphere1, &sphere2, &sphere3, &sphere4, &sphere5]);

    let camera = Camera::new(
        &Vec3::new(-2.0, 2.0, 1.0),
        &Vec3::new(0.0, 0.0, -1.0),
        &Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (nx as f32) / (ny as f32),
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let rr = rand::thread_rng().gen_range(0.0, 1.0);
                let u = ((i as f32) + rr) / (nx as f32);
                let v = ((j as f32) + rr) / (ny as f32);
                let r = camera.get_ray(u, v);
                col += &color(&r, &world, 50);
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
