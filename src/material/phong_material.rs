use math::*;
use core::*;
use bxdf::*;

#[derive(Debug)]
pub struct SpecularLighting {
    color: Color,
    exponent: f64,
}

#[derive(Debug)]
pub struct Transmission {
    pub transmissivity: f64,
    pub index_of_refraction: f64,
}

#[derive(Debug)]
pub struct PhongMaterial {
    diffuse: Color,
    specular: SpecularLighting,
    transmissivity: Transmission,
    reflectivity: f64,
}

impl PhongMaterial {
    fn new(diffuse: Color, specular: SpecularLighting, transmissivity: Transmission, reflectivity: f64) -> PhongMaterial {
        PhongMaterial {
            diffuse,
            specular,
            transmissivity,
            reflectivity,
        }
    }
}

impl Material for PhongMaterial {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        Bsdf::new(
            vec![
                Box::new(Lambertian::new(self.diffuse)),
                // TODO: Use exponent (and mixing fraction?).
                Box::new(PerfectSpecularReflection::new(self.specular.color)),
                // TODO: Use IOR (and mixing fraction?). Where does refractive color come from?
                Box::new(PerfectSpecularTransmission::new(self.diffuse)),
            ],
            Transform::new(Mat4::create_translation(-intersection.location.as_vector())),
        )
    }
}
