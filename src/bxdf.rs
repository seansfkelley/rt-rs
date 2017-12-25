use std::f64::consts::PI;
use rand::Rng;
use math::*;
use core::*;
use samples::*;

// TODO: Read up on why things are not always in the upper hemisphere.
fn same_hemisphere(v1: &Vec3, v2: &Vec3) -> bool {
    (v1.z < 0f64) == (v2.z < 0f64)
}

fn cos_theta(v: &Vec3) -> f64 {
    v.assert_normalized();
    v.z
}

fn abs_cos_theta(v: &Vec3) -> f64 {
    v.assert_normalized();
    v.z.abs()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub struct Bsdf {
    bxdfs: Vec<Box<Bxdf>>,
    world_to_local: Transform,
    eta: f64, // For refraction, I guess? A little abstraction-breaky but not terrible.
}

impl Bsdf {
    pub fn new(bxdfs: Vec<Box<Bxdf>>, world_to_local: Transform, eta: f64) -> Bsdf {
        Bsdf { bxdfs, world_to_local, eta }
    }

    pub fn evaluate(&self, w_o_world: Vec3, w_i_world: Vec3, types: Vec<BxdfType>) -> Color {
        let w_o = w_o_world.transform(&self.world_to_local);
        let w_i = w_i_world.transform(&self.world_to_local);
        w_o.assert_normalized();
        w_i.assert_normalized();

        let mut color = Color::BLACK;
        for bxdf in &self.bxdfs {
            if types.contains(&bxdf.bxdf_type()) {
                color += bxdf.evaluate(w_o, w_i);
            }
        }

        color
    }

    pub fn choose_and_evaluate(&self, w_o_world: Vec3, rng: &mut Rng, types: Vec<BxdfType>) -> (Color, f64, Vec3) {
        let w_o = w_o_world.transform(&self.world_to_local);
        w_o.assert_normalized();

        for bxdf in &self.bxdfs {
            if types.contains(&bxdf.bxdf_type()) {
                // TODO: Have to modify pdf value per pbrt, though I think that only applies when you can
                // have multiple brdfs that match.
                return bxdf.choose_and_evaluate(w_o, rng);
            }
        }

        (Color::BLACK, 0f64, Vec3::uniform(0f64))
    }
}
