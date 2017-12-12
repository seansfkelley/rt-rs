use std::rc::Rc;
use core::*;

#[derive(Debug)]
pub struct SceneObject {
    pub shape: Shape,
    pub texture: Rc<Texture>,
}

impl Geometry for SceneObject {
    fn bound(&self) -> BoundingBox {
        self.shape.bound()
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray).map(|i| {
            let material = self.texture.get_material(i.uv);
            i.with_material(material)
        })
    }
}
