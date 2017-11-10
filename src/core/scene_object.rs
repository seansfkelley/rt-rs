use std::rc::Rc;
use std::f64::consts::PI;
use super::ray::Ray;
use super::intersection::Hit;
use material::Material;
use geometry::Geometry;
use core::*;
use math::transform::Mat4;
use math::transform::IDENTITY;

#[derive(Debug)]
pub struct SceneObject {
    pub shape: Shape,
    pub material: Rc<Material>,
}

impl SceneObject {
    pub fn from_geo<G: Geometry>(geometry: &Rc<G>, material: &Rc<Material>) -> SceneObject {
        SceneObject {
            shape: Shape::new(geometry, IDENTITY),
            material: Rc::clone(material),
        }
    }

    pub fn from_shape<G: Geometry>(shape: &Shape, material: &Rc<Material>) -> SceneObject {
        SceneObject {
            shape: shape.clone(),
            material: Rc::clone(material),
        }
    }

    pub fn new<G: Geometry>(geometry: &Rc<G>, transform: Mat4, material: &Rc<Material>) -> SceneObject {
        SceneObject {
            shape: Shape::new(geometry, transform),
            material: Rc::clone(material),
        }
    }

    pub fn material(&self) -> Rc<Material> {
        self.material
    }
}

impl Geometry for SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shape.intersect(ray)
    }
}
