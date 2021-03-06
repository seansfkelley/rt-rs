use std::f64::INFINITY;
use math::*;
use core::*;

#[derive(Debug)]
pub struct DirectionalLight {
    reversed_direction: Vec3,
    radiance: Color,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, radiance: Color) -> DirectionalLight {
        DirectionalLight {
            reversed_direction: -direction.into_normalized(),
            radiance,
        }
    }
}

// pbrt pg. 621
impl Light for DirectionalLight {
    fn choose_and_sample_radiance(&self, p: Point) -> LightSample {
        let w_i = self.reversed_direction.clone();
        LightSample {
            l: self.radiance,
            w_i,
            pdf: 1f64,
            visibility_ray: Ray::finite(p, w_i, EPSILON, INFINITY),
        }
    }

    fn pdf(&self, _p: Point, _w_i: Vec3) -> f64 {
        // Delta lights are effectively impossible to sample well at random.
        0f64
    }
}
