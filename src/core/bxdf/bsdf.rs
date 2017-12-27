use rand::Rng;
use math::*;
use core::*;

pub struct Bsdf {
    bxdfs: Vec<Box<Bxdf>>,
    world_to_local: Transform,
    // eta: f64, // For refraction, I guess? A little abstraction-breaky but not terrible.
}

impl Bsdf {
    pub fn new(bxdfs: Vec<Box<Bxdf>>, world_to_local: Transform) -> Bsdf {
        Bsdf { bxdfs, world_to_local }
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

    pub fn pdf(&self, w_o_world: Vec3, w_i_world: Vec3, types: Vec<BxdfType>) -> f64 {
        let w_o = w_o_world.transform(&self.world_to_local);
        let w_i = w_i_world.transform(&self.world_to_local);
        w_o.assert_normalized();
        w_i.assert_normalized();

        let mut pdf = 0f64;
        let mut matching_bxdf_count = 0;
        for bxdf in &self.bxdfs {
            if types.contains(&bxdf.bxdf_type()) {
                pdf += bxdf.pdf(w_o, w_i);
                matching_bxdf_count += 1;
            }
        }

        if matching_bxdf_count > 0 { pdf / matching_pdf_count as f64 } else { 0f64 }
    }
}
