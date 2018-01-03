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
        // Perfect specular BxDFs are effectively impossible to sample well at random.
        Color::BLACK
    }

    fn choose_and_evaluate(&self, w_o: Vec3, _rng: &mut Rng) -> BxdfSample {
        let w_i = Vec3::new(-w_o.x, -w_o.y, w_o.z); // Reflection in the local coordinate system.
        // TODO: actually evaluate the Fresnel value to modulate the reflectance by.
        BxdfSample::new(self.reflectance, 1f64, w_i)
    }
}
