use math::*;
use core::*;

pub struct SpecularLighting {
    color: Color,
    exponent: f64,
}

pub struct Transmission {
    pub transmissivity: f64,
    pub index_of_refraction: f64,
}

pub struct PhongMaterial {
    diffuse: Color,
    specular: SpecularLighting,
    transmissivity: Transmission,
    reflectivity: f64,
}

impl Material for PhongMaterial {
    fn new(color: Color, specular: SpecularLighting, transmissivity: Transmission, reflectivity: f64) -> PhongMaterial {
        PhongMaterial {
            color,
            specular,
            transmissivity,
            reflectivity,
        }
    }

    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        Bsdf::new(
            vec![
                Lambertian::new(self.diffuse),
                // TODO: Use exponent (and mixing fraction?).
                PerfectSpecularReflection::new(self.specular.color),
                // TODO: Use IOR (and mixing fraction?). Where does refractive color come from?
                PerfectSpecularTransmission::new(self.diffuse),
            ],
            Transform::new(Mat4::create_translation(-intersection.location.as_vector())),
        )
    }
}
