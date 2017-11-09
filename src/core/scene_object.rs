use std::rc::Rc;
use super::ray::Ray;
use super::intersection::Hit;
use material::Material;

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn material(&self) -> Rc<Material>;
}
