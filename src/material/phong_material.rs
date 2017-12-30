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
            // TODO: pbrt has these two IOR flipped the other way, but why?
            bxdfs.push(Box::new(PerfectSpecularTransmission::new(self.transmission, 1f64, self.index_of_refraction)));
        }

        Bsdf::new(bxdfs, intersection)
    }
}
