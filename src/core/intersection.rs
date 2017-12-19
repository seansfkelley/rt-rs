use math::*;
use material::*;

#[derive(Debug, Clone, Copy)]
pub struct Uv(pub f64, pub f64);

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f64,
    pub location: Point,
    pub normal: Normal,
    pub shading_normal: Option<Normal>,
    pub uv: Option<Uv>,
    pub material: Option<Material>,
}

impl Intersection {
    pub fn with_material(self, material: Material) -> Intersection {
        Intersection {
            distance: self.distance,
            location: self.location,
            normal: self.normal,
            shading_normal: self.shading_normal,
            uv: self.uv,
            material: Some(material),
        }
    }
}
