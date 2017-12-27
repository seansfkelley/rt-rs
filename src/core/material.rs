use super::intersection::Intersection;
use super::bsdf::Bsdf;

pub trait Material {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf;
}
