use color::{Color, BLACK};
use core::*;
use material::*;

#[derive(Debug, Clone)]
pub struct CheckerboardTexture {
    pub checks_per_unit: u32,
    pub color_a: Color,
    pub color_b: Color,
}

impl Texture for CheckerboardTexture {
    fn get_material(&self, intersection: &Intersection) -> Material {
        let check_size = 1f64 / self.checks_per_unit as f64;
        let color =
            if (intersection.uv.0 / check_size) as u32 % 2 == (intersection.uv.1 / check_size) as u32 % 2 {
                self.color_a
            } else {
                self.color_b
            };
        // TODO: How to do more properly??
        Material {
            ambient: color * 0.1f64,
            diffuse: color,
            specular: SpecularLighting(BLACK, 0f64),
            transmission: None,
            reflectivity: 0f64,
        }
    }
}
