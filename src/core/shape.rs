use std::sync::Arc;
use core::*;

#[derive(Debug, Clone)]
pub struct Shape {
    geometry: Arc<Geometry>,
    object_to_world: Transform,
    bound: BoundingBox,
}

impl Shape {
    pub fn new(geometry: Arc<Geometry>, object_to_world: Transform) -> Shape {
        let bound = geometry.bound().transform(&object_to_world);
        Shape {
            geometry,
            object_to_world,
            bound,
        }
    }
}

impl Geometry for Shape {
    fn bound(&self) -> BoundingBox {
        self.bound.clone()
    }

    fn intersect(&self, world_ray: &Ray) -> Option<Intersection> {
        self.geometry.intersect(&world_ray.clone().invert_transform(&self.object_to_world))
            .map(|i| i.transform(&self.object_to_world))
    }
}
