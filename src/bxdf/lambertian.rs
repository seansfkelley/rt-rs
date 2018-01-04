use std::f64::consts::PI;
use math::*;
use core::*;

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

    fn evaluate(&self, w_o: Vec3, w_i: Vec3) -> Color {
        // I don't know if this is right, but pbrt doesn't really address this: Lambertian shading should (?)
        // not contribute if eitehr vector is not on the outside of the surface (like the inside of a triangle
        // mesh).
        if w_i.z > 0f64 && w_o.z > 0f64 {
            self.reflectance / PI
        } else {
            Color::BLACK
        }
    }
}
