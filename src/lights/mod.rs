use math::*;
use core::*;

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
