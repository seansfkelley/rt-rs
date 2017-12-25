use math::*;
use core::*;

pub enum LightType {
    Delta,
    Area,
}

pub trait Light {
    pub fn is_delta_light(&self) -> bool;
    // We are not on the light, so pick a point we can see from `from` and sample it.
    pub fn choose_and_L(&self, from: Point) -> (Color, f64, Vec3);
}

pub trait AreaLight: Light {
    // We collided with the light, so compute the radiance it emits directly.
    pub fn L(&self, w_o: Vec3, at: Point, normal: Normal) -> Color;
}

// pbrt pg. 424, 428
pub enum SpectrumType {
    Diffuse,
    GlossySpecular,
    PerfectSpecular,
}

// pbrt pg. 428
pub enum DirectionType {
    Reflective,
    Transmissive,
}

// TODO: Maybe a pair of enums isn't the best type?
pub type BxdfType = (DirectionType, SpectrumType);

pub trait Bxdf {
    pub fn type(&self) -> BxdfType;
    pub fn evaluate(&self, w_o: Vec3, w_i: Vec3) -> Color;
    pub fn choose_and_evaluate(&self, w_o: Vec3) -> (Color, f64, Vec3);
}

pub struct Bsdf {
    bxdfs: Vec<<Bxdf>>,
    eta: f64, // For refraction, I guess? A little abstraction-breaky but not terrible.
}

impl Bsdf {
    pub fn new(bxdfs: Vec<Box<Bxdf>>, eta: f64) {
        Bsdf { bxdfs, eta }
    }

    pub fn evaluate(&self, w_o: Vec3, w_i: Vec3, types: Vec<BxdfType>) -> Color {

    }

    pub fn choose_and_evaluate(&self, w_o: Vec3, types: Vec<BxdfType>) -> (Color, f64, Vec3) {

    }
}
