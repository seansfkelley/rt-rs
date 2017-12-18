use std::path::Path;
use image::{RgbImage, open as openImage};
use color::{Color};
use core::*;
use material::*;
use util::UvMap;

#[derive(Debug, Clone)]
pub enum LightingFacet {
    Ambient,
    Diffuse,
    Specular,
    Reflectivity,
}

impl LightingFacet {
    fn get_facet<'a>(&self, image_texture: &'a ImageTexture) -> &'a ImageTextureFacet {
        match *self {
            LightingFacet::Ambient => &image_texture.ambient,
            LightingFacet::Diffuse => &image_texture.diffuse,
            LightingFacet::Specular => &image_texture.specular,
            LightingFacet::Reflectivity => &image_texture.reflectivity,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImageTextureFacet {
    Image(RgbImage),
    Constant(Color),
    Reference(LightingFacet, f64),
}

impl ImageTextureFacet {
    pub fn get_color(&self, uv: Uv, image_texture: &ImageTexture) -> Color {
        match self {
            &ImageTextureFacet::Constant(color) => color,
            &ImageTextureFacet::Image(ref image) => image.get_color(uv),
            &ImageTextureFacet::Reference(ref facet, fraction) =>
                fraction * facet.get_facet(image_texture).get_color(uv, image_texture),
        }
    }

    pub fn from_path(path: &Path) -> ImageTextureFacet {
        ImageTextureFacet::Image(
            match openImage(path) {
                Ok(img) => { img.to_rgb() }
                Err(reason) => { panic!("could not open image at {:?}: {:?}", path, reason); }
            })
    }
}

#[derive(Debug, Clone)]
pub struct ImageTexture {
    ambient: ImageTextureFacet,
    diffuse: ImageTextureFacet,
    specular: ImageTextureFacet,
    reflectivity: ImageTextureFacet,
}

impl ImageTexture {
    pub fn new(
        ambient: ImageTextureFacet,
        diffuse: ImageTextureFacet,
        specular: ImageTextureFacet,
        reflectivity: ImageTextureFacet,
    ) -> ImageTexture {
        ImageTexture { ambient, diffuse, specular, reflectivity }
    }
}

impl Texture for ImageTexture {
    fn get_material(&self, intersection: &Intersection) -> Material {
        Material {
            ambient: self.ambient.get_color(intersection.uv, self),
            diffuse: self.diffuse.get_color(intersection.uv, self),
            // TODO: How to do more properly??
            specular: SpecularLighting(self.specular.get_color(intersection.uv, self), 20f64),
            transmission: None,
            // TODO: How to do more properly??
            reflectivity: self.reflectivity.get_color(intersection.uv, self).average(),
        }
    }
}
