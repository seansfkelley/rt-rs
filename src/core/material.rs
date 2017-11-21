use std::path::Path;
use std::fmt::Debug;
use image::{RgbImage, Pixel, open as openImage};
use color::{Color, BLACK};
use core::*;

type SpecularExponent = f64;

#[derive(Debug, Clone, Copy)]
pub struct SpecularLighting(pub Color, pub SpecularExponent);

#[derive(Debug, Clone, Copy)]
pub struct Transmission {
    pub transmissivity: f64,
    pub index_of_refraction: f64,
}

type Reflectivity = f64;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: SpecularLighting,
    pub transmission: Option<Transmission>,
    pub reflectivity: Reflectivity,
}

pub trait Texture: Debug {
    fn get_material(&self, intersection: &Intersection) -> Material;
}

#[derive(Debug)]
pub struct ConstantTexture {
    pub material: Material,
}

impl Texture for ConstantTexture {
    fn get_material(&self, _intersection: &Intersection) -> Material {
        self.material
    }
}

#[derive(Debug, Clone)]
pub struct ImageTexture {
    image: RgbImage,
    reflectivity: f64,
}

impl ImageTexture {
    pub fn from_path(p: &Path, reflectivity: f64) -> ImageTexture {
        let image = match openImage(p) {
            Ok(img) => { img.to_rgb() }
            Err(reason) => { panic!("could not open image at {:?}: {:?}", p, reason); }
        };
        let (width, height) = image.dimensions();
        assert_eq!(width, height);
        ImageTexture {
            image,
            reflectivity,
        }
    }
}

impl Texture for ImageTexture {
    fn get_material(&self, intersection: &Intersection) -> Material {
        let (width, height) = self.image.dimensions();
        let pixel = self.image.get_pixel((width as f64 * intersection.uv.0) as u32, (height as f64 * intersection.uv.1) as u32);
        let rgb = pixel.channels();
        let color = Color::new(rgb[0] as f64 / 255f64, rgb[1] as f64 / 255f64, rgb[2] as f64 / 255f64);

        // TODO: How to do more properly??
        Material {
            ambient: color * 0.1f64,
            diffuse: color,
            specular: SpecularLighting(BLACK, 0f64),
            transmission: None,
            reflectivity: self.reflectivity,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

