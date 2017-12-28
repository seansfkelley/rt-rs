use math::*;
use core::*;
use bxdf::*;

#[derive(Debug)]
pub struct FlatMaterial {
    color: Color,
}

impl FlatMaterial {
    pub fn new(color: Color) -> FlatMaterial {
        FlatMaterial {
            color,
        }
    }
}

impl Material for FlatMaterial {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        Bsdf::new(
            vec![Box::new(Lambertian::new(self.color))],
            Transform::new(Mat4::create_translation(-intersection.location.as_vector())),
        )
    }
}
