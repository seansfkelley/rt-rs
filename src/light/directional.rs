use math::*;
use core::*;

#[derive(Debug)]
pub struct DirectionalLight {
    direction: Vec3,
    radiance: Color,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, radiance: Color) -> DirectionalLight {
        DirectionalLight {
            direction: direction.into_normalized(),
            radiance,
        }
    }
}

// pbrt pg. 621
impl Light for DirectionalLight {
    fn choose_and_sample_L(&self, p: Point) -> LightSample {
        let w_i = self.direction.clone();
        LightSample {
            color: self.radiance,
            w_i,
            pdf: 1f64,
            visibility_ray: Ray::half_infinite(p, w_i),
        }
    }
}
