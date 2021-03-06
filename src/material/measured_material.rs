use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
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
    pub samples: Arc<PointKdTree<MeasuredSample>>,
    pub smoothing: usize,
}

impl Material for MeasuredMaterial {
    fn get_bsdf(&self, intersection: &Intersection) -> Bsdf {
        Bsdf::new(vec![
            Box::new(Measured::new(Arc::clone(&self.samples)))
        ], intersection)
    }
}

impl MeasuredMaterial {
    pub fn from(path: &Path, scale: f64, smoothing: usize) -> MeasuredMaterial {
        let samples = strip_comments(read_file_contents(path)).as_str()
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .map(|line| {
                let v: Vec<f64> = line.split(",").map(|v| f64::from_str(v).unwrap()).collect();
                RawPhiThetaMeasuredSample {
                    theta_i: v[0],
                    phi_i: v[1],
                    theta_o: v[2],
                    phi_o: v[3],
                    color: Color::new(v[4], v[5], v[6]) * scale,
                }
            })
            // pbrt pg. 465
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
            samples: Arc::new(PointKdTree::from(samples)),
            smoothing,
        }
    }
}
