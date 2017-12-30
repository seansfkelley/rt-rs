use noise::{ Perlin, Seedable, NoiseModule };
use core::*;

#[derive(Debug)]
pub struct Wood {
    perlin: Perlin,
    color_one: Color,
    color_two: Color,
    scale: f64,
}

impl Wood {
    pub fn new(seed: usize, color_one: Color, color_two: Color, scale: f64) -> Wood {
        Wood {
            perlin: Perlin::new().set_seed(seed),
            color_one,
            color_two,
            scale,
        }
    }
}

impl Texture for Wood {
    fn get_color(&self, uv: Option<Uv>) -> Color {
        match uv {
            Some(Uv(u, v)) => {
                let point = intersection.location * self.scale;
                let grain = (self.perlin.get([point.x, point.y, point.z]) * 5f64).fract();
                self.color_one * grain + self.color_two * (1f64 - grain)
            }
            None => { Color::BLACK }
        }
    }
}
