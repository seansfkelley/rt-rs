use std::fmt::Debug;
use math::*;
use super::ray::Ray;
use super::color::Color;

#[derive(Debug)]
pub enum LightType {
    Delta(Box<Light>),
    Area(Box<AreaLight>),
}

impl Light for LightType {
    fn choose_and_sample_radiance(&self, p: Point) -> LightSample {
        match self {
            &LightType::Delta(ref light) => {
                light.choose_and_sample_radiance(p)
            }
            &LightType::Area(ref light) => {
                light.choose_and_sample_radiance(p)
            }
        }
    }

    fn pdf(&self, p: Point, w_i: Vec3) -> f64 {
        match self {
            &LightType::Delta(ref light) => {
                light.pdf(p, w_i)
            }
            &LightType::Area(ref light) => {
                light.pdf(p, w_i)
            }
        }
    }
}

pub struct LightSample {
    pub l: Color,
    pub w_i: Vec3,
    pub pdf: f64,
    pub visibility_ray: Ray,
}

pub trait Light: Sync + Debug {
    fn choose_and_sample_radiance(&self, p: Point) -> LightSample;
    fn pdf(&self, p: Point, w_o: Vec3) -> f64;
}

pub trait AreaLight: Light {
    fn sample_radiance(&self, p: Point, n: Normal, world_w_o: Vec3) -> Color;
}
