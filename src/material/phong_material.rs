use core::*;
use bxdf::*;

#[derive(Debug)]
pub struct PhongMaterial {
    pub diffuse: Box<Texture>,
    pub specular: Box<Texture>, // TODO: Glossy specular term.
    pub reflection: Box<Texture>,
    pub transmission: Box<Texture>,
    pub index_of_refraction: f64,
}

impl Material for PhongMaterial {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        let uv = intersection.uv;
        let mut bxdfs: Vec<Box<Bxdf>> = vec![];

        {
            let diffuse = self.diffuse.get_color(uv);
            if diffuse.is_nonzero() {
                bxdfs.push(Box::new(Lambertian::new(diffuse)));
            }
        }

        {
            let reflection = self.reflection.get_color(uv);
            if reflection.is_nonzero() {
                bxdfs.push(Box::new(PerfectSpecularReflection::new(reflection)));
            }
        }

        {
            let transmission = self.transmission.get_color(uv);
            if transmission.is_nonzero() {
                // TODO: pbrt has these two IOR flipped the other way, but why?
                bxdfs.push(Box::new(PerfectSpecularTransmission::new(transmission, 1f64, self.index_of_refraction)));
            }
        }

        Bsdf::new(bxdfs, intersection)
    }
}
