use core::*;
use bxdf::*;

#[derive(Debug)]
pub struct FlatMaterial {
    pub texture: Box<Texture>,
}

impl Material for FlatMaterial {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        Bsdf::new(vec![
            Box::new(Lambertian::new(self.texture.get_color(intersection.uv)))
        ], intersection)
    }
}
