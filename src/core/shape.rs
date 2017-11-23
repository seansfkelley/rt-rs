use std::rc::Rc;
use super::ray::Ray;
use super::intersection::Intersection;
use geometry::Geometry;
use core::*;

#[derive(Debug, Clone)]
pub struct Shape {
    geometry: Rc<Geometry>,
    object_to_world: Transform,
}

impl Shape {
    pub fn new(geometry: Rc<Geometry>, object_to_world: Transform) -> Shape {
        Shape { geometry, object_to_world }
    }
}

impl Geometry for Shape {
    fn intersect(&self, world_ray: &Ray) -> Option<Intersection> {
        let ref object_ray = world_ray.invert_transform(&self.object_to_world);
        match self.geometry.intersect(object_ray) {
            Some(object_space_intersection) => Some(Intersection {
                distance: object_space_intersection.distance,
                location: object_space_intersection.location.transform(&self.object_to_world),
                normal: object_space_intersection.normal.transform(&self.object_to_world),
                uv: object_space_intersection.uv,
            }),
            None => None,
        }
    }
}


