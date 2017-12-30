use std::fmt::Debug;
use super::intersection::Intersection;
use super::bsdf::Bsdf;

pub trait Material: Sync + Send + Debug {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf;
}
