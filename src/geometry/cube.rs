use core::*;
use super::Geometry;

#[derive(Debug)]
pub struct Cube { }

impl Geometry for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        None
    }
}
