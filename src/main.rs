use std::fs::File;
use std::io::Write;
mod ray;
mod vec3;
use ray::Ray;
use vec3::Vec3;

fn color(&ray: Ray) -> Vec3 {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut data = format!("P3\n{} {} \n255\n", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..(ny - 1)).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r=
            let r = (i as f32) / (nx as f32);
            let g = (j as f32) / (ny as f32);
            let b = 0.2;
            data.push_str(&format!(
                "{} {} {}\n",
                ((255.99 * r) as i32),
                ((255.99 * g) as i32),
                ((255.99 * b) as i32)
            ));
        }
    }
    println!("{:?}", vec3::Vec3::new(1.0, 2.0, 3.0));
    let mut f = File::create("test.ppm").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}
