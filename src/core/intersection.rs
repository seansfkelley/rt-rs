use math::*;
use core::*;

pub type Uv = (f64, f64);

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f64,
    pub location: Point,
    pub normal: Normal,
    pub uv: Uv,
    pub material: Option<Material>,
}

const NUDGE_FACTOR: f64 = 1e-10f64;
impl Intersection {
    pub fn nudged_location(&self, normal: Normal) -> Point {
        self.location + (normal * NUDGE_FACTOR).as_vector()
    }

    pub fn with_material(self, material: Material) -> Intersection {
        Intersection {
            distance: self.distance,
            location: self.location,
            normal: self.normal,
            uv: self.uv,
            material: Some(material),
        }
    }
}
