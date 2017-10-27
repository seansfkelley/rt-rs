use objects::*;
use color::{ Color, BLACK };
use std::path::Path;
use std::fmt::Debug;
use image;
use image::{ RgbImage, Pixel };

pub trait Material: Debug {
    fn reflectivity(&self) -> f64;
    fn get_color(&self, ray: &Ray, intersection: &Intersection, lights: &Vec<&Light>) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct PhongMaterial {
    ambient: Color,
    diffuse: Color,
    specular: Color,
    specular_exponent: f64,
    reflectivity: f64,
}

impl PhongMaterial {
    pub fn plastic<'a>(color: Color) -> PhongMaterial {
        PhongMaterial {
            ambient: color * 0.1,
            diffuse: color,
            specular: color * 0.5,
            specular_exponent: 2f64,
            reflectivity: 0.1,
        }
    }

    pub fn mirror<'a>() -> PhongMaterial {
        PhongMaterial {
            ambient: Color { r: 0f64, g: 0f64, b: 0f64 },
            diffuse: Color { r: 0.05f64, g: 0.05f64, b: 0.05f64 },
            specular: Color { r: 1f64, g: 1f64, b: 1f64 },
            specular_exponent: 10f64,
            reflectivity: 0.95,
        }
    }
}

impl Material for PhongMaterial {
    fn reflectivity(&self) -> f64 { self.reflectivity }

    fn get_color(&self, ray: &Ray, intersection: &Intersection, lights: &Vec<&Light>) -> Color {
        let mut color = self.ambient;
        for light in lights {
            let light_direction = (light.position - intersection.location).as_unit_vector();
            let diffuse_illumination = self.diffuse * light.color * intersection.normal.dot(light_direction).max(0f64);
            let specular_illumination = self.specular * light.color * intersection.normal.dot((light_direction - ray.direction).as_unit_vector()).max(0f64).powf(self.specular_exponent);
            color = color + diffuse_illumination + specular_illumination;
        }
        color
    }
}

#[derive(Debug, Clone)]
pub struct ImageTextureMaterial {
    image: RgbImage,
    reflectivity: f64,
}

impl ImageTextureMaterial {
    fn from(p: &Path) -> ImageTextureMaterial {
        let image = image::open(p).unwrap().to_rgb();
        let (width, height) = image.dimensions();
        assert_eq!(width, height);
        ImageTextureMaterial {
            image,
            reflectivity: 0.1,
        }
    }
}

impl Material for ImageTextureMaterial {
    fn reflectivity(&self) -> f64 { self.reflectivity }

    fn get_color(&self, _ray: &Ray, intersection: &Intersection, _lights: &Vec<&Light>) -> Color {
        let (width, height) = self.image.dimensions();
        let pixel = self.image.get_pixel((width as f64 * intersection.uv.0) as u32, (height as f64 * intersection.uv.1) as u32);
        let rgb = pixel.channels();
        Color::new(rgb[0] as f64 / 255f64, rgb[1] as f64 / 255f64, rgb[2] as f64 / 255f64)
    }
}

#[derive(Debug)]
pub struct UnionMaterial {
    materials: Vec<(Box<Material>, f64)>,
}

impl Material for UnionMaterial {
    fn reflectivity(&self) -> f64 { 0f64 }

    fn get_color(&self, ray: &Ray, intersection: &Intersection, lights: &Vec<&Light>) -> Color {
        let mut color = BLACK;
        for &(ref material, blending_factor) in &self.materials {
            color = color + material.get_color(ray, intersection, lights) * blending_factor;
        }
        color
    }
}
