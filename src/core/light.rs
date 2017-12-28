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
    fn choose_and_sample_L(&self, p: Point) -> LightSample {
        match self {
            &LightType::Delta(ref light) => {
                light.choose_and_sample_L(p)
            }
            &LightType::Area(ref light) => {
                light.choose_and_sample_L(p)
            }
        }
    }
}

pub struct LightSample {
    pub color: Color,
    pub w_i: Vec3,
    pub pdf: f64,
    pub visibility_ray: Ray,
}

pub trait Light: Sync + Debug {
    // We are not on the light, so pick a point we can see from `p` and sample it.
    fn choose_and_sample_L(&self, p: Point) -> LightSample;
}

pub trait AreaLight: Light {
    // We collided with the light, so compute the radiance it emits directly.
    fn L(&self, p: Point, n: Normal, world_w_o: Vec3) -> Color;
}
