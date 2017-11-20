use math::*;
use material::Material;
use std::rc::Rc;

pub type Uv = (f64, f64);

pub struct Intersection {
    pub distance: f64,
    pub location: Point,
    pub normal: Normal,
    pub uv: Uv,
}

const NUDGE_FACTOR: f64 = 1e-10f64;
impl Intersection {
    pub fn nudge(&self, normal: Normal) -> Intersection {
        Intersection {
            distance: self.distance,
            location: self.location + (normal * NUDGE_FACTOR).as_vector(),
            normal: self.normal,
            uv: self.uv,
        }
    }
}

pub struct Hit {
    pub enter: Option<Intersection>,
    pub exit: Intersection,
}

pub struct MaterialHit {
    pub hit: Hit,
    pub material: Rc<Material>,
}
