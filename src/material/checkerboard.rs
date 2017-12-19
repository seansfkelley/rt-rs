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
        let color = match intersection.uv {
            Some(Uv(u, v)) => {
                if (u / check_size) as u32 % 2 == (v / check_size) as u32 % 2 {
                    self.color_a
                } else {
                    self.color_b
                }
            },
            None => { panic!("cannot compute checkerboard texture for intersection without uv"); },
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
