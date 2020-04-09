mod vec3;

pub struct Ray {
    origin: vec3::Vec3,
    direction: vec3::Vec3,
}

impl Ray {
    fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray { origin, direction }
    }
    fn parameterization(&self, t: f32) -> Vec3 {
        t * self + origin
    }
}
