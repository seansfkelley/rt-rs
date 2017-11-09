use std::rc::Rc;

use core::*;
use material::Material;
use transform::Mat4;

#[derive(Debug)]
pub struct Cube {
    transform: Mat4,
    material: Rc<Material>,
}

impl SceneObject for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        None
    }

    fn material(&self) -> Rc<Material> { Rc::clone(&self.material) }
}
