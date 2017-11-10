use std::rc::Rc;
use std::f64::consts::PI;
use super::ray::Ray;
use super::intersection::Hit;
use material::Material;
use geometry::Geometry;
use core::*;

#[derive(Debug, Clone)]
pub struct Shape {
    geometry: Rc<Geometry>,
    transform: Transform,
}

impl Shape {
    pub fn new<G: Geometry + 'static>(geometry: &Rc<G>, transform: Transform) -> Shape {
        Shape {
            geometry: Rc::<G>::clone(geometry),
            transform,
        }
    }

    fn get_intersection(&self, object_intersection: Intersection, world_ray: &Ray, object_ray: &Ray) -> Intersection {
        let world_location = object_intersection.location.object_to_world(&self.transform);

        Intersection {
            distance: object_intersection.distance,
            location: world_location,
            // http://www.unknownroad.com/rtfm/graphics/rt_normals.html
            normal: object_intersection.normal.object_to_world(&self.transform),
            uv: object_intersection.uv,
        }
    }
}

impl Geometry for Shape {
    fn intersect(&self, world_ray: &Ray) -> Option<Hit> {
        let ref object_ray = world_ray.world_to_object(&self.transform);
        let object_space_hit_option = self.geometry.intersect(object_ray);
        object_space_hit_option.map(|object_space_hit| {
            Hit {
                enter: object_space_hit.enter.map(|enter| self.get_intersection(enter, world_ray, object_ray)),
                exit: self.get_intersection(object_space_hit.exit, world_ray, object_ray),
                debug: object_space_hit.debug,
            }
        })
    }
}


