use rand::Rng;
use math::*;
use super::bxdf::*;
use super::color::Color;
use super::intersection::Intersection;

pub struct Bsdf {
    bxdfs: Vec<Box<Bxdf>>,
    normal: Normal,
    primary_tangent: Vec3,
    secondary_tangent: Vec3,
    // eta: f64, // TODO: What does pbrt use this field for? Refraction?
}

impl Bsdf {
    pub fn new(bxdfs: Vec<Box<Bxdf>>, intersection: &Intersection) -> Bsdf {
        // TODO: This is the same as the math in Renderer. Should one defer to the other?
        let normal = {
            match intersection.shading_normal {
                Some(normal) => normal,
                None => intersection.normal,
            }
        }.as_normalized();

        // TODO: We currently only support isotropic BxDFs, so the orientation of the primary and secondary
        // tangents is not relevant. In pbrt, these are computed based on the normal and dp/du.

        // TODO: We should probably compute dp/du, because this is sometimes degenerate.
        let primary_tangent = normal.cross(Vec3::X_AXIS).into_normalized();
        let secondary_tangent = normal.cross(primary_tangent).into_normalized();

        Bsdf {
            bxdfs,
            normal,
            primary_tangent,
            secondary_tangent,
        }
    }

    fn world_to_local(&self, v: &Vec3) -> Vec3 {
        Vec3::new(v.dot(&self.primary_tangent), v.dot(&self.secondary_tangent), v.dot(&self.normal))
    }

    fn local_to_world(&self, v: &Vec3) -> Vec3 {
        let (p_t, s_t, n) = (self.primary_tangent, self.secondary_tangent, self.normal);
        Vec3::new(
            p_t.x * v.x + s_t.x * v.y + n.x * v.z,
            p_t.y * v.x + s_t.y * v.y + n.y * v.z,
            p_t.z * v.x + s_t.z * v.y + n.z * v.z,
        )
    }

    pub fn evaluate(&self, w_o_world: Vec3, w_i_world: Vec3, types: &Vec<BxdfType>) -> Color {
        let w_o = self.world_to_local(&w_o_world);
        let w_i = self.world_to_local(&w_i_world);
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

    pub fn choose_and_evaluate(&self, w_o_world: Vec3, rng: &mut Rng, types: &Vec<BxdfType>) -> Option<(BxdfSample, SpectrumType)> {
        let w_o = self.world_to_local(&w_o_world);
        w_o.assert_normalized();

        for bxdf in &self.bxdfs {
            let bxdf_type = &bxdf.bxdf_type();
            if types.contains(bxdf_type) {
                // TODO: Have to modify pdf value per pbrt, though I think that only applies when you can
                // have multiple brdfs that match.
                let mut bxdf_sample = bxdf.choose_and_evaluate(w_o, rng);
                bxdf_sample.w_i = self.local_to_world(&bxdf_sample.w_i);
                return Some((bxdf_sample, bxdf_type.1));
            }
        }

        None
    }

    pub fn pdf(&self, w_o_world: Vec3, w_i_world: Vec3, types: &Vec<BxdfType>) -> f64 {
        let w_o = self.world_to_local(&w_o_world);
        let w_i = self.world_to_local(&w_i_world);
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

        if matching_bxdf_count > 0 { pdf / matching_bxdf_count as f64 } else { 0f64 }
    }
}
