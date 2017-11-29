use std::rc::Rc;
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

impl Boundable for SceneObject {
    fn bound(&self) -> BoundingBox {
        self.shape.bound()
    }
}
