use std::f64::consts::PI;
use core::Color;
use math::*;
use super::bxdf::*;

// pbrt pg. 447
pub struct Lambertian {
    reflectance: Color,
}

impl Lambertian {
    pub fn new(reflectance: Color) -> Lambertian {
        Lambertian { reflectance }
    }
}

impl Bxdf for Lambertian {
    fn bxdf_type(&self) -> BxdfType {
        (TransportType::Reflective, SpectrumType::Diffuse)
    }

    fn evaluate(&self, _w_o: Vec3, _w_i: Vec3) -> Color {
        self.reflectance / PI
    }
}
