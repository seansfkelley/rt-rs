use rand::Rng;
use core::*;
use math::*;

pub struct PerfectSpecularTransmission {
    transmittance: Color,
    eta_i: f64,
    eta_t: f64,
    // TODO: Different types of Fresnel.
}

impl PerfectSpecularTransmission {
    pub fn new(transmittance: Color, eta_i: f64, eta_t: f64) -> PerfectSpecularTransmission {
        PerfectSpecularTransmission {
            transmittance,
            eta_i,
            eta_t,
        }
    }
}

impl Bxdf for PerfectSpecularTransmission {
    fn bxdf_type(&self) -> BxdfType {
        (TransportType::Transmissive, SpectrumType::PerfectSpecular)
    }

    fn evaluate(&self, _w_o: Vec3, _w_i: Vec3) -> Color {
        // We're not even going to try: your choice of w_o/w_i needs to be so spot-on to machine epsilon
        // that we're going to assume you didn't do it and force you to use choose_and_evaluate.
        Color::BLACK
    }

    // pbrt pg. 445
    fn choose_and_evaluate(&self, w_o: Vec3, _rng: &mut Rng) -> BxdfSample {
        // TODO: Can some of this control flow be handed off to evaluate_fresnel? It's quite similar.
        let is_entering = cos_theta(&w_o) > 0f64;
        let (eta_i, eta_t) = if is_entering {
            (self.eta_i, self.eta_t)
        } else {
            (self.eta_t, self.eta_i)
        };

        let sin_i_2 = sin_theta_2(&w_o);
        let eta = eta_i / eta_t;
        let sin_t_2 = eta * eta * sin_i_2;

        if sin_t_2 > 1f64 {
            BxdfSample::new(Color::BLACK, 0f64, Vec3::uniform(0f64))
        } else {
            let cos_t = non_nan_max(0f64, 1f64 - sin_t_2).sqrt() * (if is_entering { -1f64 } else { 1f64 });
            let w_i = Vec3::new(eta * -w_o.x, eta * -w_o.y, cos_t);
            BxdfSample::new(
                (1f64 - evaluate_fresnel(cos_theta(&w_o), self.eta_i, self.eta_t)) / cos_theta(&w_i).abs() * self.transmittance,
                1f64,
                w_i,
            )
        }
    }
}

fn evaluate_fresnel(cos_i: f64, eta_i: f64, eta_t: f64) -> f64 {
    let cos_i = cos_i.clamp(-1f64, 1f64);

    let is_entering = cos_i > 0f64;
    let (eta_i, eta_t) = if is_entering {
        (eta_i, eta_t)
    } else {
        (eta_t, eta_i)
    };

    let eta = eta_i / eta_t;
    let sin_t_2 = eta * eta * non_nan_max(0f64, 1f64 - cos_i * cos_i);
    if sin_t_2 >= 1f64 {
        1f64
    } else {
        let cos_i = cos_i.abs();
        let cos_t = non_nan_max(0f64, 1f64 - sin_t_2).sqrt();
        let r_orthogonal = (eta_i * cos_i - eta_t * cos_t) / (eta_i * cos_i + eta_t * cos_t);
        let r_parallel = (eta_t * cos_i - eta_i * cos_t) / (eta_t * cos_i + eta_i * cos_t);
        (r_orthogonal * r_orthogonal + r_parallel * r_parallel) / 2f64
    }
}
