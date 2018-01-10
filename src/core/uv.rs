use std::f64::consts::PI;
use std::ops::{ Mul, Add };
use math::*;

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

const TWO_PI: f64 = PI * 2f64;

// Adapted from https://en.wikipedia.org/wiki/UV_mapping#Finding_UV_on_a_sphere.
// pbrt pg. 119 talks about this as well, but I prefer the branchless, poles-along-y-axis version here.
// Note that this function accepts a point on the sphere surface, which makes sense, and then flips the
// sign to match the formula (which wants a vector from point to origin).
pub fn sphere_uv_for_normalized_point(point: Point) -> Uv {
    let v = -point.into_vector();
    v.assert_normalized();

    Uv(
        0.5f64 - v.z.atan2(v.x) / TWO_PI,
        0.5f64 - v.y.asin() / PI,
    )
}

pub fn sphere_uv_to_polar(uv: Uv) -> (f64, f64) {
    (uv.0 * TWO_PI, uv.1 * PI)
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
