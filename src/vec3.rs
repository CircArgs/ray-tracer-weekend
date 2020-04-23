use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 {
            x: e1,
            y: e2,
            z: e3,
        }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn r(&self) -> f64 {
        self.x
    }
    pub fn g(&self) -> f64 {
        self.y
    }
    pub fn b(&self) -> f64 {
        self.z
    }
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.dot(self)
    }
    pub fn normalize(&self) -> Vec3 {
        self / self.length()
    }
    pub fn _normalize(&mut self) {
        *self = self.normalize();
    }
    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y - other.x,
        }
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        (self * other).sum()
    }
    pub fn project(&self, onto: &Vec3) -> Vec3 {
        onto * (self.dot(onto) / onto.squared_length())
    }
    pub fn rotate(&self, phi: f64, theta: f64) -> Vec3 {
        //phi φ in [0, pi] indicates a deviation in radians from the +z axis
        //theta in [0, 2pi] indicates a deviation from the +x axis in the x-y plane
        let (sin_phi, cos_phi) = phi.sin_cos();
        let (sin_theta, cos_theta) = theta.sin_cos();
        let col1 = Vec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi);
        let col2 = Vec3::new(-sin_theta * sin_phi, cos_theta * sin_phi, 0.0);
        let col3 = Vec3::new(cos_theta * cos_phi, sin_theta * cos_phi, -sin_phi);
        &(&(&col1 * self.x) + &(&col2 * self.y)) + &(&col3 * self.z)
    }

    pub fn from_spherical(radius: f64, phi: f64, theta: f64) -> Self {
        //radius ρ in [0, infinity)
        //phi φ in [0, pi] indicates a deviation in radians from the +z axis
        //theta in [0, 2pi] indicates a deviation from the +x axis in the x-y plane
        let sin_phi = phi.sin();
        Vec3::new(
            radius * sin_phi * theta.cos(),
            radius * sin_phi * theta.sin(),
            radius * phi.cos(),
        )
    }
}

impl Add<f64> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<f64> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        let temp = 1.0 / rhs;
        self * temp
    }
}

impl Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let temp = 1.0 / rhs;
        *self *= temp;
    }
}

impl DivAssign<&Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: &Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adds() {
        let temp = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(temp, &temp * 1.0);
    }
    #[test]
    fn test_normalize() {
        let mut temp = Vec3::new(1.0, 2.0, 3.0);
        let normalized = temp.normalize();
        assert_ne!(temp, normalized);
        temp._normalize();
        assert_eq!(temp, normalized);
    }

    #[test]
    fn test_cross() {
        let temp1 = Vec3::new(1.0, 0.0, 0.0);
        let temp2 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(temp1.cross(&temp2), Vec3::new(0.0, 0.0, 1.0));
    }
}
