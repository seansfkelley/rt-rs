use std::f64::consts::PI;
use math::*;
use core::*;
use super::bxdf_trig::*;

const TWO_PI: f64 = PI * 2f64;

struct RawMeasuredSample {
    w_o: Vec3,
    wi: Vec3,
    color: Color,
}

// pbrt pg. 465
#[derive(Debug)]
struct MeasuredSample {
    marschner_location: Point,
    color: Color,
}

impl Pointable for MeasuredSample {
    fn get_point(&self) -> Point {
        self.marschner_location
    }
}

// pbrt pg. 465
fn compute_marschner_location(w_o: Vec3, w_i: Vec3) -> Point {
    let delta_phi = bxdf_spherical_phi(&w_i) - bxdf_spherical_phi(&w_o);

    let clamped_delta_phi = if delta_phi < 0f64 {
        delta_phi + TWO_PI
    } else if delta_phi > TWO_PI {
        delta_phi - TWO_PI
    } else {
        delta_phi
    };

    let minimal_delta_phi = if clamped_delta_phi > PI { TWO_PI - clamped_delta_phi } else { clamped_delta_phi };

    Point::new(
        bxdf_sin_theta(&w_i) * bxdf_sin_theta(&w_o),
        minimal_delta_phi / PI,
        bxdf_cos_theta(&w_i) * bxdf_cos_theta(&w_o),
    )
}

pub struct Measured(PointKdTree<MeasuredSample>);

impl Measured {
    pub fn new(data: Vec<RawMeasuredSample>) -> Measured {
        // pbrt pg. 465
        let samples = data
            .into_iter()
            .map(|datum| MeasuredSample {
                marschner_location: compute_marschner_location(datum.w_o, datum.wi),
                color: datum.color,
            })
            .collect();
        Measured(PointKdTree::from(samples))
    }
}

impl Bxdf for Measured {
    fn bxdf_type(&self) -> BxdfType {
        // TODO: What type is it?
        (TransportType::Reflective, SpectrumType::Diffuse)
    }

    // pbrt pg. 466
    fn evaluate(&self, w_o: Vec3, w_i: Vec3) -> Color {
        let nearest = self.0.k_nearest(compute_marschner_location(w_o, w_i), 3);
        // TODO
    }
}
