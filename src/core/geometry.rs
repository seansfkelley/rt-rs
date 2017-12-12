use std::fmt::Debug;
use core::*;

pub trait Geometry: Debug {
    fn bound(&self) -> BoundingBox;
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn does_intersect(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}
