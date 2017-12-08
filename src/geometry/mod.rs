pub mod difference;
pub mod rect_prism;
pub mod sphere;
pub mod triangle_mesh;
pub mod cloth;

pub use self::difference::*;
pub use self::rect_prism::*;
pub use self::sphere::*;
pub use self::triangle_mesh::*;
pub use self::cloth::*;

use core::ray::Ray;
use core::intersection::Intersection;
use core::bounding_box::BoundingBox;
use std::fmt::Debug;

pub trait Geometry : Debug {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn bound(&self) -> BoundingBox;
}
