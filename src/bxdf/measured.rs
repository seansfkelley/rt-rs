use std::f64::consts::PI;
use std::sync::Arc;
use math::*;
use core::*;
use super::bxdf_trig::*;

const TWO_PI: f64 = PI * 2f64;

// pbrt pg. 465
#[derive(Debug)]
pub struct MeasuredSample {
    pub marschner_location: Point,
    pub color: Color,
}

impl Pointable for MeasuredSample {
    fn get_point(&self) -> Point {
        self.marschner_location
    }
}

// pbrt pg. 465
pub fn compute_marschner_location(w_o: Vec3, w_i: Vec3) -> Point {
    _compute_marschner_location(
        bxdf_sin_theta(&w_i),
        bxdf_cos_theta(&w_i),
        bxdf_spherical_phi(&w_i),
        bxdf_sin_theta(&w_o),
        bxdf_cos_theta(&w_o),
        bxdf_spherical_phi(&w_o),
    )
}

pub fn compute_marschner_location_phi_theta(theta_i: f64, phi_i: f64, theta_o: f64, phi_o: f64) -> Point {
    _compute_marschner_location(theta_i.sin(), theta_i.cos(), phi_i, theta_o.sin(), theta_o.cos(), phi_o)
}

fn _compute_marschner_location(sin_theta_i: f64, cos_theta_i: f64, phi_i: f64, sin_theta_o: f64, cos_theta_o: f64, phi_o: f64) -> Point {
    let delta_phi = phi_i - phi_o;

    let clamped_delta_phi = if delta_phi < 0f64 {
        delta_phi + TWO_PI
    } else if delta_phi > TWO_PI {
        delta_phi - TWO_PI
    } else {
        delta_phi
    };

    let minimal_delta_phi = if clamped_delta_phi > PI { TWO_PI - clamped_delta_phi } else { clamped_delta_phi };

    Point::new(
        sin_theta_i * sin_theta_o,
        minimal_delta_phi / PI,
        cos_theta_i * cos_theta_o,
    )
}

pub struct Measured(Arc<PointKdTree<MeasuredSample>>);

impl Measured {
    pub fn new(data: Arc<PointKdTree<MeasuredSample>>) -> Measured {
        Measured(data)
    }
}

impl Bxdf for Measured {
    fn bxdf_type(&self) -> BxdfType {
        (TransportType::Reflective, SpectrumType::GlossySpecular)
    }

    // pbrt pg. 466
    fn evaluate(&self, w_o: Vec3, w_i: Vec3) -> Color {
        let target_point = compute_marschner_location(w_o, w_i);
        let (color, total_weight) = self.0.k_nearest(target_point, 3)
            .into_iter()
            .map(|sample| (
                sample.color,
                (-100f64 * (sample.marschner_location - target_point).magnitude2()).exp(),
            ))
            .fold(
                (Color::BLACK, 0f64),
                |(acc_color, acc_weight), (color, weight)| (acc_color + weight * color, acc_weight + weight),
            );
        color / total_weight
    }
}
