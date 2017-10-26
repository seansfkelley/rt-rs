use std::ops::{Add, Sub, Div, Mul, Neg};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude2(self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(self) -> f64 {
        self.magnitude2().sqrt()
    }

    pub fn as_unit_vector(self) -> Vec3 {
        self / self.magnitude()
    }

}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, divisor: f64) -> Vec3 {
        Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, multiplicand: f64) -> Vec3 {
        Vec3 {
            x: self.x * multiplicand,
            y: self.y * multiplicand,
            z: self.z * multiplicand,
        }
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
