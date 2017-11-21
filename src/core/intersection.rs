use math::*;
use core::*;
use scene::Scene;

pub type Uv = (f64, f64);

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
}

pub struct Hit {
    pub enter: Option<Intersection>,
    pub exit: Intersection,
}

impl Hit {
    pub fn unwrap(self) -> CompleteHit {
        CompleteHit {
            enter: self.enter.unwrap(),
            exit: self.exit,
        }
    }
}

pub struct CompleteHit {
    pub enter: Intersection,
    pub exit: Intersection,
}

pub struct SceneObjectHit<'a> {
    pub hit: CompleteHit,
    pub scene_object: &'a SceneObject,
}

impl<'a> SceneObjectHit<'a> {
    pub fn get_color(&self, ray: &Ray, scene: &Scene, current_depth: u32) -> Color {
        self.scene_object.material.get_color(ray, self, scene, current_depth)
    }
}
