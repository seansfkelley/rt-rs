use std::f64::consts::PI;
use math::*;
use core::*;

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
