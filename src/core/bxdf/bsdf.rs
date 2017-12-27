use rand::Rng;
use core::*;
use math::*;
use super::bxdf::*;

pub struct Bsdf {
    bxdfs: Vec<Box<Bxdf>>,
    world_to_local: Transform,
    eta: f64, // For refraction, I guess? A little abstraction-breaky but not terrible.
}

impl Bsdf {
    pub fn new(bxdfs: Vec<Box<Bxdf>>, world_to_local: Transform, eta: f64) -> Bsdf {
        Bsdf { bxdfs, world_to_local, eta }
    }

    pub fn evaluate(&self, w_o_world: Vec3, w_i_world: Vec3, types: Vec<BxdfType>) -> Color {
        let w_o = w_o_world.transform(&self.world_to_local);
        let w_i = w_i_world.transform(&self.world_to_local);
        w_o.assert_normalized();
        w_i.assert_normalized();

        let mut color = Color::BLACK;
        for bxdf in &self.bxdfs {
            if types.contains(&bxdf.bxdf_type()) {
                color += bxdf.evaluate(w_o, w_i);
            }
        }

        color
    }

    pub fn choose_and_evaluate(&self, w_o_world: Vec3, rng: &mut Rng, types: Vec<BxdfType>) -> (Color, f64, Vec3) {
        let w_o = w_o_world.transform(&self.world_to_local);
        w_o.assert_normalized();

        for bxdf in &self.bxdfs {
            if types.contains(&bxdf.bxdf_type()) {
                // TODO: Have to modify pdf value per pbrt, though I think that only applies when you can
                // have multiple brdfs that match.
                return bxdf.choose_and_evaluate(w_o, rng);
            }
        }

        (Color::BLACK, 0f64, Vec3::uniform(0f64))
    }
}
