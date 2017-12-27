use math::*;
use core::*;

#[derive(Debug)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
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
}
