use std::f64::consts::PI;
use rand::Rng;
use math::*;
use core::*;

// pbrt pg. 424, 428
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumType {
    Diffuse,
    GlossySpecular,
    PerfectSpecular,
}

// pbrt pg. 428
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    Reflective,
    Transmissive,
}

// TODO: Maybe a pair of enums isn't the best type?
pub type BxdfType = (TransportType, SpectrumType);

pub trait Bxdf {
    fn bxdf_type(&self) -> BxdfType;

    fn evaluate(&self, w_o: Vec3, w_i: Vec3) -> Color;

    fn choose_and_evaluate(&self, w_o: Vec3, rng: &mut Rng) -> (Color, f64, Vec3) {
        w_o.assert_normalized();
        let mut w_i = sample_hemisphere_cosine(rng);
        w_i.assert_normalized();
        if w_o.z < 0f64 {
            w_i.z = -w_i.z;
        }
        (self.evaluate(w_o, w_i), self.pdf(w_o, w_i), w_i)
    }

    // TODO: Read up on why this is the default.
    fn pdf(&self, w_o: Vec3, w_i: Vec3) -> f64 {
        if same_hemisphere(&w_o, &w_i) {
            abs_cos_theta(&w_i) / PI
        } else {
            0f64
        }
    }
}
