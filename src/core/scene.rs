use std::sync::Arc;
use super::bounding_box::BoundingBox;
use super::geometry::Geometry;
use super::intersection::Intersection;
use super::kd_tree::KdTree;
use super::light::LightType;
use super::ray::Ray;
use super::shape::Shape;
use super::material::Material;

#[derive(Debug)]
pub struct SceneObject {
    pub shape: Shape,
    pub material: Arc<Material>,
}

impl Geometry for SceneObject {
    fn bound(&self) -> BoundingBox {
        self.shape.bound()
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray).map(|i| i.with_material(Arc::clone(&self.material)))
    }
}

pub struct Scene {
    pub objects: KdTree<SceneObject>,
    pub lights: Vec<LightType>,
}
