use std::fs::File;
use std::io::Write;
mod camera;
mod ray;
mod shapes;
mod vec3;
use camera::Camera;
use ray::Ray;
use shapes::*;
use vec3::Vec3;

fn color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    &(&Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Vec3::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut data = format!("P3\n{} {} \n255\n", nx, ny);
    let sphere1 = Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = Intersectables::new(vec![&sphere1, &sphere2]);
    let camera = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r = camera.get_ray(u, v);
            let mut col = color(&r);

            match world.intersect(&r) {
                Some(hit) => {
                    let n = (&hit.point + &Vec3::new(0.0, 0.0, 1.0)).normalize();
                    col = &(&n + 1.0) * 0.5;
                }
                _ => {}
            }
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
