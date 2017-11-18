use std::path::Path;
use std::fmt::Debug;
use image::{RgbImage, Pixel, open as openImage};
use color::{Color, BLACK, WHITE};
use intersection::Intersection;
use scene::Scene;
use core::*;
use math::*;

type SpecularExponent = f64;

#[derive(Debug, Clone, Copy)]
pub struct SpecularLighting(pub Color, pub SpecularExponent);

type Reflectivity = f64;

#[derive(Debug, Clone, Copy)]
pub struct ReflectiveLighting(pub Color, pub Reflectivity);

pub trait Material: Debug {
    fn get_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene, current_depth: u32) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct FlatMaterial {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: SpecularLighting,
}

impl Material for FlatMaterial {
    fn get_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene, _current_depth: u32) -> Color {
        scene.get_visible_lights(&intersection.nudge(-ray.direction))
            .iter()
            .fold(BLACK, |color, light| {
                let light_direction = (light.position - intersection.location).as_normalized();
                let normalized_normal = intersection.normal.as_normalized();
                let diffuse_illumination = self.diffuse * light.color * normalized_normal.dot(&light_direction).max(0f64);
                let specular_illumination = self.specular.0 * light.color * normalized_normal.dot(&(light_direction - ray.direction).as_normalized()).max(0f64).powf(self.specular.1);
                color + diffuse_illumination + specular_illumination
            })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReflectiveMaterial {
    pub flat_material: FlatMaterial,
    pub reflective: ReflectiveLighting,
}

impl ReflectiveMaterial {
    pub fn new(ambient: Color, diffuse: Color, specular: SpecularLighting, reflective: ReflectiveLighting) -> ReflectiveMaterial {
        ReflectiveMaterial {
            flat_material: FlatMaterial { ambient, diffuse, specular },
            reflective,
        }
    }

}

impl Material for ReflectiveMaterial {
    fn get_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene, current_depth: u32) -> Color {
        let mut color = self.flat_material.get_color(ray, intersection, scene, current_depth);

        let reflectivity = self.reflective.1;

        if reflectivity > 0f64 {
            let new_origin = ray.at(intersection.distance);
            let new_direction = ray.direction.reflect(intersection.normal.as_vector());
            let new_ray = Ray::new(new_origin, new_direction);
            // TODO: I think we're supposed to multiply the reflectivity by the color of the surface...?
            color = (1f64 - reflectivity) * color + reflectivity * scene.get_color(&new_ray, current_depth + 1)
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
    pub fn from_path(p: &Path, reflectivity: f64) -> ImageTextureMaterial {
        let image = match openImage(p) {
            Ok(img) => { img.to_rgb() }
            Err(reason) => { panic!("could not open image at {:?}: {:?}", p, reason); }
        };
        let (width, height) = image.dimensions();
        assert_eq!(width, height);
        ImageTextureMaterial {
            image,
            reflectivity,
        }
    }
}

impl Material for ImageTextureMaterial {
    fn get_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene, current_depth: u32) -> Color {
        let (width, height) = self.image.dimensions();
        let pixel = self.image.get_pixel((width as f64 * intersection.uv.0) as u32, (height as f64 * intersection.uv.1) as u32);
        let rgb = pixel.channels();
        let color = Color::new(rgb[0] as f64 / 255f64, rgb[1] as f64 / 255f64, rgb[2] as f64 / 255f64);

        // TODO: How to do more properly??
        ReflectiveMaterial::new(
            color * 0.1f64,
            color,
            SpecularLighting(BLACK, 0f64),
            ReflectiveLighting(color, self.reflectivity),
        ).get_color(ray, intersection, scene, current_depth)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CheckerboardMaterial {
    pub checks_per_unit: u32,
    pub color_a: Color,
    pub color_b: Color,
}

impl Material for CheckerboardMaterial {
    fn get_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene, current_depth: u32) -> Color {
        let check_size = 1f64 / self.checks_per_unit as f64;
        let color =
            if (intersection.uv.0 / check_size) as u32 % 2 == (intersection.uv.1 / check_size) as u32 % 2 {
                self.color_a
            } else {
                self.color_b
            };
        // TODO: How to do more properly??
        ReflectiveMaterial::new(
            color * 0.1f64,
            color,
            SpecularLighting(BLACK, 0f64),
            ReflectiveLighting(BLACK, 0f64),
        ).get_color(ray, intersection, scene, current_depth)
    }
}
