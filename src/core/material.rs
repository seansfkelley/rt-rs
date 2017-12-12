use std::fmt::Debug;
use color::{Color, BLACK};
use core::*;

type SpecularExponent = f64;

#[derive(Debug, Clone)]
pub struct SpecularLighting(pub Color, pub SpecularExponent);

#[derive(Debug, Clone)]
pub struct Transmission {
    pub transmissivity: f64,
    pub index_of_refraction: f64,
}

type Reflectivity = f64;

#[derive(Debug, Clone)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: SpecularLighting,
    pub transmission: Option<Transmission>,
    pub reflectivity: Reflectivity,
}

pub trait Texture: Debug {
    fn get_material(&self, uv: Uv) -> Material;
}

#[derive(Debug)]
pub struct ConstantTexture {
    pub material: Material,
}

impl Texture for ConstantTexture {
    fn get_material(&self, _uv: Uv) -> Material {
        self.material.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CheckerboardTexture {
    pub checks_per_unit: u32,
    pub color_a: Color,
    pub color_b: Color,
}

impl Texture for CheckerboardTexture {
    fn get_material(&self, uv: Uv) -> Material {
        let check_size = 1f64 / self.checks_per_unit as f64;
        let color =
            if (uv.0 / check_size) as u32 % 2 == (uv.1 / check_size) as u32 % 2 {
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

