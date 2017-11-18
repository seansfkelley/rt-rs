use std::rc::Rc;
use super::ray::Ray;
use super::intersection::Hit;
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

    fn get_intersection(&self, object_intersection: Intersection) -> Intersection {
        let world_location = object_intersection.location.transform(&self.object_to_world);

        Intersection {
            distance: object_intersection.distance,
            location: world_location,
            normal: object_intersection.normal.transform(&self.object_to_world),
            uv: object_intersection.uv,
        }
    }
}

impl Geometry for Shape {
    fn intersect(&self, world_ray: &Ray) -> Option<Hit> {
        let ref object_ray = world_ray.invert_transform(&self.object_to_world);
        let object_space_hit_option = self.geometry.intersect(object_ray);
        object_space_hit_option.map(|object_space_hit| {
            Hit {
                enter: object_space_hit.enter.map(|enter| self.get_intersection(enter)),
                exit: self.get_intersection(object_space_hit.exit),
            }
        })
    }
}


