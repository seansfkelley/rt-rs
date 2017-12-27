use super::bxdf::*;
use super::super::intersection::Intersection;

pub trait Material {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf;
}
