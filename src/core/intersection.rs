use std::rc::Rc;

use math::*;
use core::*;

pub type Uv = (f64, f64);

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub distance: f64,
    pub location: Point,
    pub normal: Normal,
    pub uv: Uv,
}

const NUDGE_FACTOR: f64 = 1e-10f64;
impl Intersection {
    pub fn nudged_location(&self, normal: Normal) -> Point {
        self.location + (normal * NUDGE_FACTOR).as_vector()
    }
}

pub struct TexturedIntersection {
    pub intersection: Intersection,
    pub texture: Rc<Texture>,
}
