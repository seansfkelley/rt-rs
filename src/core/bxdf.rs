use std::f64::consts::PI;
use rand::Rng;
use math::*;
use super::color::Color;

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

pub struct BxdfSample {
    color: Color,
    pdf: f64,
    w_i: Vec3,
}

impl BxdfSample {
    pub fn new(color: Color, pdf: f64, w_i: Vec3) -> BxdfSample {
        BxdfSample {
            color,
            pdf,
            w_i,
        }
    }
}

pub trait Bxdf {
    fn bxdf_type(&self) -> BxdfType;

    fn evaluate(&self, w_o: Vec3, w_i: Vec3) -> Color;

    fn choose_and_evaluate(&self, w_o: Vec3, rng: &mut Rng) -> BxdfSample {
        w_o.assert_normalized();
        let mut w_i = sample_hemisphere_cosine(rng);
        w_i.assert_normalized();
        if w_o.z < 0f64 {
            w_i.z = -w_i.z;
        }
        BxdfSample::new(self.evaluate(w_o, w_i), self.pdf(w_o, w_i), w_i)
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

// pbrt pg. 693
pub fn variance_power_heuristic(f_pdf: f64, f_samples: usize, g_pdf: f64, g_samples: usize) -> f64 {
    let f = f_pdf * f_samples as f64;
    let g = g_pdf * g_samples as f64;
    (f * f) / (f * f + g * g)
}
