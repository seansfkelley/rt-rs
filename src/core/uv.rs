use std::f64::consts::PI;
use std::ops::{ Mul, Add };
use math::*;
use super::color::Color;

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

pub trait UvMap {
    fn get_color(&self, uv: Uv) -> Color;
}

// Adapted from https://en.wikipedia.org/wiki/UV_mapping#Finding_UV_on_a_sphere.
// pbrt pg. 119 talks about this as well, but I prefer the branchless, poles-along-y-axis version here.
// Note that this function accepts a point on the sphere surface, which makes sense, and then flips the
// sign to match the formula (which wants a vector from point to origin).
pub fn sphere_uv_for_normalized_point(point: Point) -> Uv {
    const TWO_PI: f64 = PI * 2f64;

    let v = -point.into_vector();
    v.assert_normalized();

    Uv(
        0.5f64 + v.z.atan2(v.x) / TWO_PI,
        0.5f64 - v.y.asin() / PI,
    )
}
