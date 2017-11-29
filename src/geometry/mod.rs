pub mod difference;
pub mod rect_prism;
pub mod sphere;
pub mod triangle_mesh;

pub use self::difference::*;
pub use self::rect_prism::*;
pub use self::sphere::*;
pub use self::triangle_mesh::*;

use core::ray::Ray;
use core::intersection::Intersection;
use core::bounding_box::Boundable;
use std::fmt::Debug;

pub trait Geometry : Debug + Boundable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
