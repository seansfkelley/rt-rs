use noise::{Perlin, Seedable, NoiseModule};
use core::*;
use material::*;

#[derive(Debug)]
pub struct Wood {
    perlin: Perlin,
    color_one: Color,
    color_two: Color,
    scale: f64,
    reflectivity: f64,
    specular_exponent: f64,
}

impl Wood {
    pub fn new(seed: usize, color_one: Color, color_two: Color, scale: f64, reflectivity: f64, specular_exponent: f64) -> Wood {
        Wood {
            perlin: Perlin::new().set_seed(seed),
            color_one,
            color_two,
            scale,
            reflectivity,
            specular_exponent,
        }
    }
}

impl Texture for Wood {
    fn get_material(&self, intersection: &Intersection) -> Material {
        let point = intersection.location * self.scale;
        let grain =
            (self.perlin.get([point.x, point.y, point.z]) * 5f64).fract();
        let color = self.color_one * grain + self.color_two * (1f64 - grain);

        Material {
            ambient: color * 0.1f64,
            diffuse: color,
            specular: SpecularLighting(color * self.reflectivity, self.specular_exponent),
            reflectivity: self.reflectivity,
            transmission: None,
        }
    }
}
