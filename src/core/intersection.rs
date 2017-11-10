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

pub struct Hit {
    pub enter: Option<Intersection>,
    pub exit: Intersection,
    pub debug: bool,
}

pub struct MaterialHit {
    pub hit: Hit,
    pub material: Rc<Material>,
}

impl Hit {
    #[allow(dead_code)]
    pub fn debug(self, debug: bool) -> Hit {
        Hit {
            enter: self.enter,
            exit: self.exit,
            debug,
        }
    }
}
