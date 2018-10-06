use std::path::Path;
use core::*;
use bxdf::*;
use math::*;

pub struct RawMeasuredSample {
    w_o: Vec3,
    wi: Vec3,
    color: Color,
}

#[derive(Debug)]
pub struct MeasuredMaterial {
    pub samples: PointKdTree<MeasuredSample>,
}

impl Material for MeasuredMaterial {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        Bsdf::new(vec![
            Box::new(Measured::new(self.samples))
        ], intersection)
    }
}

impl MeasuredMaterial {
    pub fn from(path: &Path) -> MeasuredMaterial {
        match openImage(path) {
            Ok(img) => { img.to_rgb() }
            Err(reason) => { panic!("could not open file at {:?}: {:?}", path, reason); }
        }
    }

    pub fn new(data: Vec<RawMeasuredSample>) -> MeasuredMaterial {
        // pbrt pg. 465
        let samples = data
            .into_iter()
            .map(|datum| MeasuredSample {
                marschner_location: compute_marschner_location(datum.w_o, datum.wi),
                color: datum.color,
            })
            .collect();
        MeasuredMaterial {
            samples: PointKdTree::from(samples),
        }
    }
}
