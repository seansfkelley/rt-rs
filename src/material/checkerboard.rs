use color::{Color, BLACK};
use core::*;
use material::*;

#[derive(Debug, Clone)]
pub struct CheckerboardTexture {
    pub checks_u: u32,
    pub checks_v: u32,
    pub color_one: Color,
    pub color_two: Color,
}

impl Texture for CheckerboardTexture {
    fn get_material(&self, intersection: &Intersection) -> Material {
        let color = match intersection.uv {
            Some(Uv(u, v)) => {
                if (u * self.checks_u as f64) as u32 % 2 == (v * self.checks_v as f64) as u32 % 2 {
                    self.color_one
                } else {
                    self.color_two
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
