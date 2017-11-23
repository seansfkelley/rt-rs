use std::rc::Rc;
use super::ray::Ray;
use super::intersection::Intersection;
use material::Texture;
use geometry::Geometry;
use core::*;

#[derive(Debug)]
pub struct SceneObject {
    pub shape: Shape,
    pub texture: Rc<Texture>,
}

impl Geometry for SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray)
    }
}
