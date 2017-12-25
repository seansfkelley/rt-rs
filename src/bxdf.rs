use std::f64::consts::PI;
use rand::Rng;
use math::*;
use core::*;

pub enum LightType {
    Delta,
    Area,
}

pub trait Light {
    fn is_delta_light(&self) -> bool;
    // We are not on the light, so pick a point we can see from `from` and sample it.
    fn choose_and_L(&self, from: Point) -> (Color, f64, Vec3);
}

pub trait AreaLight: Light {
    // We collided with the light, so compute the radiance it emits directly.
    fn L(&self, w_o: Vec3, at: Point, normal: Normal) -> Color;
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
    fn type(&self) -> BxdfType;
    fn evaluate(&self, w_o: Vec3, w_i: Vec3) -> Color;
    fn choose_and_evaluate(&self, w_o: Vec3, rng: ) -> (Color, f64, Vec3) {
        *wi = CosineSampleHemisphere(u1, u2);
        if (wo.z < 0.) wi->z *= -1.f;
        *pdf = Pdf(wo, *wi);
        return f(wo, *wi);
    }
}

// pbrt pg. 447
pub struct Lambertian {
    reflectance: Color,
}

impl Bxdf for Lambertian {
    pub fn type(&self) -> BxdfType {
        (Diffuse, Reflective)
    }

    pub fn evaluate(&self, _w_o: Vec3, _w_i: Vec3) -> Color {
        self.reflectance / PI
    }
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
