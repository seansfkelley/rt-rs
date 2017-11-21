use math::*;
use core::*;
use scene::Scene;

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
    pub fn nudge(&self) -> Intersection {
        Intersection {
            distance: self.distance,
            location: self.location + (self.normal * NUDGE_FACTOR).as_vector(),
            normal: self.normal,
            uv: self.uv,
        }
    }

    pub fn nega_nudge(&self) -> Intersection {
        Intersection {
            distance: self.distance,
            location: self.location - (self.normal * NUDGE_FACTOR).as_vector(),
            normal: self.normal,
            uv: self.uv,
        }
    }
}

pub struct Hit {
    pub enter: Option<Intersection>,
    pub exit: Intersection,
}

impl Hit {
    pub fn get_first_intersection(&self) -> Intersection {
        *self.enter.as_ref().unwrap_or(&self.exit)
    }
}

pub struct SceneObjectHit<'a> {
    pub hit: Hit,
    pub scene_object: &'a SceneObject,
}
