use std::sync::Arc;
use math::*;
use super::uv::Uv;
use super::material::Material;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f64,
    pub location: Point,
    pub normal: Normal,
    pub shading_normal: Option<Normal>,
    pub uv: Option<Uv>,
    pub material: Option<Arc<Material>>,
}

impl Intersection {
    pub fn with_material(self, material: Arc<Material>) -> Intersection {
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
