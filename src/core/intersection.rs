use std::ops::{ Mul, Add };
use math::*;
use material::*;

#[derive(Debug, Clone, Copy)]
pub struct Uv(pub f64, pub f64);

impl Mul<f64> for Uv {
    type Output = Uv;

    fn mul(self, other: f64) -> Uv {
        Uv(self.0 * other, self.1 * other)
    }
}

impl Add for Uv {
    type Output = Uv;

    fn add(self, other: Uv) -> Uv {
        Uv(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f64,
    pub location: Point,
    pub normal: Normal,
    pub shading_normal: Option<Normal>,
    pub uv: Option<Uv>,
    pub material: Option<Material>,
}

impl Intersection {
    pub fn with_material(self, material: Material) -> Intersection {
        Intersection {
            distance: self.distance,
            location: self.location,
            normal: self.normal,
            shading_normal: self.shading_normal,
            uv: self.uv,
            material: Some(material),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_uv() {
        let sum = Uv(1f64, 2f64) + Uv(10f64, 20f64);
        assert_eq!(sum.0, 11f64);
        assert_eq!(sum.1, 22f64);
    }

    #[test]
    fn it_should_multiply_uv() {
        let product = Uv(1f64, 2f64) * 10f64;
        assert_eq!(product.0, 10f64);
        assert_eq!(product.1, 20f64);
    }
}
