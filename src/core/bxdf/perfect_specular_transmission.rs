use rand::Rng;
use core::*;
use math::*;
use super::bxdf::*;

pub struct PerfectSpecularTransmission {
    transmittance: Color,
    // TODO: Different types of Fresnel.
}

impl PerfectSpecularTransmission {
    pub fn new(transmittance: Color) -> PerfectSpecularTransmission {
        PerfectSpecularTransmission { transmittance }
    }
}

impl Bxdf for PerfectSpecularTransmission {
    fn bxdf_type(&self) -> BxdfType {
        (TransportType::Transmissive, SpectrumType::PerfectSpecular)
    }

    fn evaluate(&self, _w_o: Vec3, _w_i: Vec3) -> Color {
        // We're not even going to try: your choice of w_o/w_i needs to be so spot-on to machine epsilon
        // that we're going to assume you didn't do it and force you to use choose_and_evaluate.
        Color::BLACK
    }

    fn choose_and_evaluate(&self, w_o: Vec3, _rng: &mut Rng) -> (Color, f64, Vec3) {
        // TODO.
        (Color::BLACK, 0f64, Vec3::uniform(0f64))
    }
}
