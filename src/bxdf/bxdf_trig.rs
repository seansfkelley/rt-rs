use std::f64::consts::PI;
use math::*;

// TODO: Use these everywhere they should be used in the various BXDFs. Do they need anymore hinting
// to be inlined appropriately?

pub fn bxdf_cos_theta(direction_vector: &Vec3) -> f64 {
    direction_vector.z
}

pub fn bxdf_sin_theta(direction_vector: &Vec3) -> f64 {
    (1f64 - bxdf_cos_theta(direction_vector).powi(2)).sqrt()
}

pub fn bxdf_spherical_phi(direction_vector: &Vec3) -> f64 {
    let phi = direction_vector.y.atan2(direction_vector.x);
    if phi < 0f64 { phi + 2f64 * PI } else { phi }
}
