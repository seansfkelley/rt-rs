use math::*;
use core::*;
use bxdf::*;

#[derive(Debug)]
pub struct PhongMaterial {
    pub diffuse: Color,
    pub specular: Color,
    pub reflection: Color,
    pub transmission: Color,
    pub index_of_refraction: f64,
}

impl Material for PhongMaterial {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        let mut bxdfs: Vec<Box<Bxdf>> = vec![];

        if self.diffuse.is_nonzero() {
            bxdfs.push(Box::new(Lambertian::new(self.diffuse)));
        }

        if self.specular.is_nonzero() {
            bxdfs.push(Box::new(PerfectSpecularReflection::new(self.specular)));
        }

        if self.transmission.is_nonzero() {
            bxdfs.push(Box::new(PerfectSpecularTransmission::new(self.transmission, self.index_of_refraction)));
        }

        Bsdf::new(
            bxdfs,
            Transform::new(Mat4::create_translation(-intersection.location.as_vector())),
        )
    }
}
