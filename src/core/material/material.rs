use core::*;

pub trait Material {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf;
}
