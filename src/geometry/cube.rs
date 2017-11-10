use std::rc::Rc;

use core::*;
use material::Material;
use transform::Mat4;
use super::Geometry;

#[derive(Debug)]
pub struct Cube { }

impl Geometry for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        None
    }
}
