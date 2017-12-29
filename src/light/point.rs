use math::*;
use core::*;

#[derive(Debug)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

// pbrt pg. 610
impl Light for PointLight {
    fn choose_and_sample_L(&self, p: Point) -> LightSample {
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

    fn pdf (&self, p: Point, w_i: Vec3) -> f64 {
        // We assume that arbitrary point's we're given are never going to hit the light.
        0f64
    }
}
