use std::rc::Rc;
use std::f64::consts::PI;
use super::ray::Ray;
use super::intersection::Hit;
use material::Material;
use geometry::Geometry;
use core::*;
use math::transform::Mat4;

#[derive(Debug, Clone)]
pub struct Shape {
    geometry: Rc<Geometry>,
    transform: Mat4,
    // derivative, but should be cached
    transpose_transform: Mat4,
    inverse_transform: Mat4,
    inverse_transform_without_translation: Mat4,
}

impl Shape {
    pub fn new<G: Geometry + 'static>(geometry: &Rc<G>, transform: Mat4) -> Shape {
        let inverse_transform = transform.invert().unwrap();
        Shape {
            geometry: Rc::<G>::clone(geometry),
            transform,
            transpose_transform: transform.transpose(),
            inverse_transform,
            inverse_transform_without_translation: inverse_transform.without_translation(),
        }
    }

    fn get_intersection(&self, object_intersection: Intersection, world_ray: &Ray, object_ray: &Ray) -> Intersection {
        let world_location = self.transform * object_intersection.location;

        Intersection {
            distance: world_location.dot(world_ray.direction),
            location: world_location,
            // http://www.unknownroad.com/rtfm/graphics/rt_normals.html
            normal: (self.transpose_transform * object_intersection.normal).as_unit_vector(),
            uv: object_intersection.uv,
        }
    }
}

impl Geometry for Shape {
    fn intersect(&self, world_ray: &Ray) -> Option<Hit> {
        let ref object_ray = world_ray.transform(self.inverse_transform, self.inverse_transform_without_translation);
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


