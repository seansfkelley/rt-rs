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
    fn choose_and_sample_radiance(&self, p: Point) -> LightSample {
        let (w_i, distance) = {
            let difference = self.position - p;
            (difference.as_normalized(), difference.magnitude())
        };
        LightSample {
            l: self.intensity / (distance * distance),
            w_i,
            pdf: 1f64,
            visibility_ray: Ray::finite(p, w_i, EPSILON, distance),
        }
    }

    fn pdf(&self, _p: Point, _w_i: Vec3) -> f64 {
        // Delta lights are effectively impossible to sample well at random.
        0f64
    }
}
