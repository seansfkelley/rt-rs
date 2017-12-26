use std::sync::Arc;
use super::bounding_box::BoundingBox;
use super::geometry::Geometry;
use super::intersection::Intersection;
use super::kd_tree::KdTree;
use super::light::Light;
use super::ray::Ray;
use super::shape::Shape;
use material::Texture;

#[derive(Debug)]
pub struct SceneObject {
    pub shape: Shape,
    pub texture: Arc<Texture>,
}

impl Geometry for SceneObject {
    fn bound(&self) -> BoundingBox {
        self.shape.bound()
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray).map(|i| {
            let material = self.texture.get_material(&i);
            i.with_material(material)
        })
    }
}

pub struct Scene {
    pub objects: KdTree<SceneObject>,
    pub lights: Vec<Light>,
}
