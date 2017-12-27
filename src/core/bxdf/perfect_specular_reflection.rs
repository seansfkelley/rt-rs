use rand::Rng;
use core::*;
use math::*;

pub struct PerfectSpecularReflection {
    reflectance: Color,
    // TODO: Different types of Fresnel.
}

impl PerfectSpecularReflection {
    pub fn new(reflectance: Color) -> PerfectSpecularReflection {
        PerfectSpecularReflection { reflectance }
    }
}

impl Bxdf for PerfectSpecularReflection {
    fn bxdf_type(&self) -> BxdfType {
        (TransportType::Reflective, SpectrumType::PerfectSpecular)
    }

    fn evaluate(&self, _w_o: Vec3, _w_i: Vec3) -> Color {
        // We're not even going to try: your choice of w_o/w_i needs to be so spot-on to machine epsilon
        // that we're going to assume you didn't do it and force you to use choose_and_evaluate.
        Color::BLACK
    }

    fn choose_and_evaluate(&self, w_o: Vec3, _rng: &mut Rng) -> (Color, f64, Vec3) {
        let w_i = Vec3::new(-w_o.x, -w_o.y, w_o.z); // Remember: local coordinate system. Reflection is easy.
        // TODO: actually evaluate the Fresnel value to modulate the reflectance by.
        (self.reflectance, 1f64, w_i)
    }
}
