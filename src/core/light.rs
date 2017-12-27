use math::*;
use super::color::Color;
use super::ray::Ray;

pub struct LightSample {
    pub color: Color,
    pub w_i: Vec3,
    pub pdf: f64,
    pub visibility_ray: Ray,
}

pub trait Light {
    #[allow(non_snake_case)]
    fn sample_L(&self, p: Point) -> LightSample;
}

#[derive(Debug)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

// pbrt pg. 610
impl Light for PointLight {
    fn sample_L(&self, p: Point) -> LightSample {
        let (w_i, distance) = {
            let difference = self.position - p;
            (difference.as_normalized(), difference.magnitude())
        };
        LightSample {
            color: self.intensity / (distance * distance),
            w_i,
            pdf: 1f64,
            visibility_ray: Ray::finite(p,  w_i, 0f64, distance),
        }
    }
}
