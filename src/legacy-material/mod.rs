pub mod image;
pub mod checkerboard;
pub mod wood;

pub use self::image::*;
pub use self::checkerboard::*;
pub use self::wood::*;

use std::fmt::Debug;
use color::{Color};
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

pub trait Texture: Debug + Send + Sync {
    fn get_material(&self, intersection: &Intersection) -> Material;
}

#[derive(Debug)]
pub struct ConstantTexture {
    pub material: Material,
}

impl Texture for ConstantTexture {
    fn get_material(&self, _intersection: &Intersection) -> Material {
        self.material.clone()
    }
}

