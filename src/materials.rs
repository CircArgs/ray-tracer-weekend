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
        let target = hit.normal().direction() + &rand_in_unit_sphere();
        Ray::new(&hit.point, &target)
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
        let normal = *hit.normal().direction();
        Ray::new(&hit.point, &reflect(ray_in.direction(), &normal, self.fuzz))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    albedo: Vec3,
    pub refraction_index: f32,
    pub fuzz: f32,
}

impl Dielectric {
    pub fn albedo(&self) -> &Vec3 {
        &self.albedo
    }
    pub fn new(refraction_index: f32, fuzz: f32) -> Self {
        Dielectric {
            albedo: Vec3::from_float(1.0),
            refraction_index,
            fuzz,
        }
    }
}

impl Material for Dielectric {
    fn albedo(&self) -> &Vec3 {
        &self.albedo
    }
    fn collide(&self, ray_in: &Ray, hit: &Hit) -> Ray {
        let normal = hit.normal();
        // println!("initial normal {:?}", normal.direction());
        let mut proj_length = normal.direction().dot(ray_in.direction());
        // println!("dot prod {:?}", proj_length);
        let (outward_normal, ni_over_nt, cosine) = if proj_length > 0.0 {
            (-*normal.direction(), self.refraction_index, {
                let cos = proj_length * self.refraction_index;
                (1.0 - self.refraction_index * self.refraction_index * (1.0 - cos * cos)).sqrt()
            })
        } else {
            (
                *normal.direction(),
                1.0 / self.refraction_index,
                -proj_length,
            )
        };

        let refracted = refract(ray_in.direction(), &outward_normal, ni_over_nt);
        // println!("refracted {:?}", refracted);
        let mut reflect_prob = 1.0;
        match refracted {
            None => {}
            Some(_) => {
                let ret = schlick(cosine, self.refraction_index);
                // println!("schlick {}", ret);
                reflect_prob = ret;
            }
        }
        if rand::thread_rng().gen::<f32>() < reflect_prob {
            // println!("refl");
            return Ray::new(
                &hit.point,
                &reflect(ray_in.direction(), &outward_normal, self.fuzz),
            );
        }
        // println!("refr");
        Ray::new(&hit.point, &refracted.unwrap())
    }
}

fn schlick(cosine: f32, index: f32) -> f32 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0 * r0;
    let ret = r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    if ret < 0.0 {
        // println!("{}", ret);
    }
    ret
}

pub fn reflect(ray_in: &Vec3, normal: &Vec3, fuzz: f32) -> Vec3 {
    let projection_length = ray_in.dot(normal);
    let proj = normal * projection_length;
    let refl = ray_in - &(&proj * 2.0);
    if fuzz == 0.0 {
        return refl;
    }
    &refl + &(&rand_in_unit_sphere() * fuzz)
}

pub fn refract(uv: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    // ni * sin(i) = nt * sin(t)
    // sin(t) = sin(i) * (ni / nt)
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = &(&(uv - &(n * dt)) * ni_over_nt) - &(n * discriminant.sqrt());
        Some(refracted)
    } else {
        None
    }
}

// pub fn refract(ray_in: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
//     // println!("normal {:?}", normal);
//     // println!("ray {:?}", ray_in);
//     // println!("ratio {:?}", ni_over_nt);
//     //snell's law: n*sin(theta)=n' * sin(theta')
//     //we'll use n, theta as the incoming (incident) values
//     //since normal direction is a unit vector and ray_in direction is a unit vector their dot product is the cosine of their subtending angle
//     //- because we assume normal is ourtward facing and incident ray is going in
//     let projection_length = ray_in.dot(normal);
//     //here the discriminant tells us if we have TIR.
//     //its square root times the reversed normal will be part of the construction of the refraction ray
//     //hence we need to be able to sqrt it and so it needs to be nonnegative
//     //this is 1.0 - refraction_index_ratio^2 *  sin(theta)^2
//     //(1.0-proj_length*proj_length) = sin(theta)^2 by the trig id from pythagorean thm
//     let discriminant =
//         1.0 - ni_over_nt * ni_over_nt * (1.0 - projection_length * projection_length);
//     // println!("refr normal {:?}", normal);
//     // println!("disc refr {}", discriminant);
//     if discriminant < 0.0 {
//         None
//     } else {
//         //projection of incident ray onto opposite facing normal
//         let p = normal * projection_length;
//         //othorgonal component of incident ray
//         let a = ray_in + &p;
//         //othorgonal component of refracted ray
//         let b = &a * ni_over_nt;
//         //projection of refracted ray onto inward facing normal
//         let pp = normal * -discriminant.sqrt();
//         Some(&pp - &b)
//     }
// }
