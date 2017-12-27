use math::*;
use core::*;

pub struct FlatMaterial {
    color: Color,
}

impl Material for FlatMaterial {
    fn new(color: Color) -> FlatMaterial {
        FlatMaterial {
            color,
        }
    }

    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        Bsdf::new(
            vec![Lambertian::new(color)],
            Transform::new(Mat4::create_translation(-intersection.location.as_vector())),
        )
    }
}
