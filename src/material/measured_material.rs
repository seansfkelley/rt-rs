use std::path::Path;
use core::*;
use bxdf::*;
use file_utils::*;

struct RawPhiThetaMeasuredSample {
    theta_i: f64,
    phi_i: f64,
    theta_o: f64,
    phi_o: f64,
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
        let contents = strip_comments(read_file_contents(path));
        // TODO: Read each line, split, then turn into a sample thing.
    }

    pub fn new(data: Vec<RawPhiThetaMeasuredSample>) -> MeasuredMaterial {
        // pbrt pg. 465
        let samples = data
            .into_iter()
            .map(|datum| MeasuredSample {
                marschner_location: compute_marschner_location_phi_theta(
                    datum.theta_i,
                    datum.phi_i,
                    datum.theta_o,
                    datum.phi_o,
                ),
                color: datum.color,
            })
            .collect();
        MeasuredMaterial {
            samples: PointKdTree::from(samples),
        }
    }
}
