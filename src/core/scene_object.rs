use std::rc::Rc;
use super::ray::Ray;
use super::intersection::Hit;
use material::Material;
use geometry::Geometry;
use core::*;

#[derive(Debug)]
pub struct SceneObject {
    pub shape: Shape,
    pub material: Rc<Material>,
}

impl Geometry for SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shape.intersect(ray)
    }
}
