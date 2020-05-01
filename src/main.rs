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
    let nx = 800;
    let ny = 400;
    let ns = 100;
    let mut data = format!("P3\n{} {} \n255\n", nx, ny);
    let material1 = Lambertian::new(&Vec3::new(0.8, 0.2, 0.5));
    let material2 = Lambertian::new(&Vec3::new(0.5, 0.8, 0.0));
    let material3 = Metal::new(&Vec3::new(0.5, 0.5, 0.5), 0.0);
    let material4 = Dielectric::new(1.5, 0.0);
    let sphere1 = Sphere::new(&Vec3::new(0.0, 1.0, -1.0), 1.0, &material1);
    let sphere2 = Sphere::new(&Vec3::new(0.0, -1000.0, -1.0), 1000.0, &material2);
    let sphere3 = Sphere::new(&Vec3::new(1.0, 1.0, -1.0), 1.0, &material3);
    let sphere4 = Sphere::new(&Vec3::new(-1.0, 1.0, -1.0), 1.0, &material4);
    let sphere5 = Sphere::new(&Vec3::new(-1.0, 1.0, -1.0), -0.98, &material4);
    let mut world = Intersectables::new(vec![&sphere1, &sphere2, &sphere3, &sphere4, &sphere5]);

    let mut materials: Vec<Box<dyn Material>> =
        vec![Box::new(Lambertian::new(&Vec3::new(0.5, 0.5, 0.5)))];

    let mut rng = rand::thread_rng();
    for _ in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            if choose_mat < 0.33 {
                // println!("lamb");
                materials.push(Box::new(Lambertian::new(&Vec3::new(
                    rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                ))));
            } else if choose_mat < 0.67 {
                // println!("metal");
                materials.push(Box::new(Metal::new(
                    &(&(&Vec3::new(
                        rng.gen_range(0.0, 1.0),
                        rng.gen_range(0.0, 1.0),
                        rng.gen_range(0.0, 1.0),
                    ) + 1.0)
                        * 0.5),
                    rng.gen_range(0.0, 1.0) * 0.5,
                )));
            } else {
                materials.push(Box::new(Dielectric::new(
                    rng.gen_range(1.3, 1.5),
                    rng.gen_range(0.0, 1.0) * 0.05,
                )));
            }
        }
    }
    let mut objects = vec![];
    let mut i = 0;
    for a in -11..11 {
        for b in -11..11 {
            let radius = 0.3 + rng.gen_range(-0.25, 0.25);
            let center = Vec3::new(
                (a as f32) + 0.9 * rng.gen_range(0.0, 1.0),
                radius,
                (b as f32) + 0.9 * rng.gen_range(0.0, 1.0),
            );
            objects.push(Sphere::new(&center, radius, &*materials[i]));
            i += 1;
        }
    }
    for i in 0..objects.len() {
        world.push(&objects[i]);
    }
    let camera = Camera::new(
        &Vec3::new(3.0, 3.0, 2.0),
        &Vec3::new(0.0, 0.0, -1.0),
        &Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (nx as f32) / (ny as f32),
        0.05,
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let rr: f32 = rng.gen();
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
