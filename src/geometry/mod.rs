pub mod difference;
pub mod sphere;
pub mod triangle_mesh;

pub use self::difference::*;
pub use self::sphere::*;
pub use self::triangle_mesh::*;

use core::ray::Ray;
use core::intersection::Hit;
use std::fmt::Debug;

pub trait Geometry : Debug {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}
