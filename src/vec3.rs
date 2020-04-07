use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(e1: f32, e2: f32, e3: f32) -> Self {
        Vec3 {
            x: e1,
            y: e2,
            z: e3,
        }
    }
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.x
    }
    fn z(&self) -> f32 {
        self.x
    }
    fn r(&self) -> f32 {
        self.x
    }
    fn g(&self) -> f32 {
        self.x
    }
    fn b(&self) -> f32 {
        self.x
    }
    fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    fn squared_length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(&self, rhs: f32) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(&self, rhs: f32) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<f32> for Vec3 {
    type Output = Vec3;
    fn add(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl AddAssign<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(&mut self, rhs: f32) -> Self {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(&self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(&self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<f32> for Vec3 {
    type Output = Vec3;
    fn sub(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl SubAssign<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(&mut self, rhs: f32) -> Self {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(&self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(&self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    type Output = Vec3;
    fn mul(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(&mut self, rhs: f32) -> Self {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(&self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(&self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    type Output = Vec3;
    fn div(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl DivAssign<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(&mut self, rhs: f32) -> Self {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
