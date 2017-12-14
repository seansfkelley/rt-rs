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
        let ref object_ray = world_ray.invert_transform(&self.object_to_world);
        match self.geometry.intersect(object_ray) {
            Some(object_space_intersection) => Some(Intersection {
                distance: object_space_intersection.distance,
                location: object_space_intersection.location.transform(&self.object_to_world),
                normal: object_space_intersection.normal.transform(&self.object_to_world),
                shading_normal: object_space_intersection.shading_normal.map(|n| n.transform(&self.object_to_world)),
                uv: object_space_intersection.uv,
                material: None,
            }),
            None => None,
        }
    }
}
