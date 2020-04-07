use std::fs::File;
use std::io::Write;
mod vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut data = format!("P3\n{} {} \n255\n", nx, ny);
    for j in (0..(ny - 1)).rev() {
        for i in 0..nx {
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
