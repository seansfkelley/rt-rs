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

impl Intersection {
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
