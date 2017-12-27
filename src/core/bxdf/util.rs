use math::*;

// TODO: Read up on why things are not always in the upper hemisphere.
pub fn same_hemisphere(v1: &Vec3, v2: &Vec3) -> bool {
    (v1.z < 0f64) == (v2.z < 0f64)
}

pub fn cos_theta(v: &Vec3) -> f64 {
    v.assert_normalized();
    v.z
}

pub fn abs_cos_theta(v: &Vec3) -> f64 {
    v.assert_normalized();
    v.z.abs()
}
