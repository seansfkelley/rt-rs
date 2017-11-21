use std::path::Path;
use std::fmt::Debug;
use image::{RgbImage, Pixel, open as openImage};
use color::{Color, BLACK, WHITE};
use scene::Scene;
use core::*;
use math::*;
use util::Clamp;
use std::rc::Rc;

type SpecularExponent = f64;

#[derive(Debug, Clone, Copy)]
pub struct SpecularLighting(pub Color, pub SpecularExponent);

type Reflectivity = f64;

pub trait Material: Debug {
    fn get_color(&self, ray: &Ray, hit: &SceneObjectHit, scene: &Scene, current_depth: u32) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct FlatMaterial {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: SpecularLighting,
}

impl Material for FlatMaterial {
    fn get_color(&self, ray: &Ray, hit: &SceneObjectHit, scene: &Scene, current_depth: u32) -> Color {
        let ref intersection = hit.hit.get_first_intersection();
        scene.get_visible_lights(intersection.nudge().location)
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
pub struct ReflectiveMaterial { }

impl ReflectiveMaterial {
    pub fn new(ambient: Color, diffuse: Color, specular: SpecularLighting, reflectivity: Reflectivity) -> MaterialComposition {
        MaterialComposition::new()
            .compose(&Rc::new(FlatMaterial { ambient, diffuse, specular }), 1f64 - reflectivity)
            .compose(&Rc::new(ReflectiveMaterial { }), reflectivity)
    }

}

impl Material for ReflectiveMaterial {
    fn get_color(&self, ray: &Ray, hit: &SceneObjectHit, scene: &Scene, current_depth: u32) -> Color {
        let ref intersection = hit.hit.get_first_intersection();
        let new_origin = ray.at(intersection.distance);
        let new_direction = ray.direction.reflect(intersection.normal.as_vector());
        let new_ray = Ray::new(new_origin, new_direction);
        scene.get_color(&new_ray, current_depth + 1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TransmissiveMaterial {
    pub index_of_refraction: f64,
    // TODO: occlusion
}

impl TransmissiveMaterial {
    fn refract(&self, ray: &Ray, intersection: &Intersection, eta: f64) -> Ray {
        let cosi = ray.direction.dot(&intersection.normal).clamp(-1f64, 1f64);
        let k = 1f64 - eta * eta * (1f64 - cosi * cosi);
        if k < 0f64 {
            // TOTAL INTERNAL REFLECTION (」゜ロ゜)」
            // TODO: make sure it good
            Ray::new(intersection.location, ray.direction.reflect(intersection.normal.as_vector()))
        } else {
            let direction = ray.direction * eta + (intersection.normal * (eta * cosi - k.sqrt())).as_vector();
            let origin = intersection.nega_nudge().location;
            Ray::new(origin, direction.as_normalized())
        }
    }
}

impl Material for TransmissiveMaterial {
    fn get_color(&self, ray: &Ray, hit: &SceneObjectHit, scene: &Scene, current_depth: u32) -> Color {
        let ref intersection = hit.hit.get_first_intersection();
        let inside = hit.hit.enter.is_none();
        let eta = if inside { self.index_of_refraction } else { 1f64 / self.index_of_refraction };
        let refracted_ray = self.refract(ray, intersection, eta);
        scene.get_color(&refracted_ray, current_depth + 1)
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
    fn get_color(&self, ray: &Ray, hit: &SceneObjectHit, scene: &Scene, current_depth: u32) -> Color {
        let ref intersection = hit.hit.get_first_intersection();
        let (width, height) = self.image.dimensions();
        let pixel = self.image.get_pixel((width as f64 * intersection.uv.0) as u32, (height as f64 * intersection.uv.1) as u32);
        let rgb = pixel.channels();
        let color = Color::new(rgb[0] as f64 / 255f64, rgb[1] as f64 / 255f64, rgb[2] as f64 / 255f64);

        // TODO: How to do more properly??
        ReflectiveMaterial::new(
            color * 0.1f64,
            color,
            SpecularLighting(BLACK, 0f64),
            self.reflectivity,
        ).get_color(ray, hit, scene, current_depth)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CheckerboardMaterial {
    pub checks_per_unit: u32,
    pub color_a: Color,
    pub color_b: Color,
}

impl Material for CheckerboardMaterial {
    fn get_color(&self, ray: &Ray, hit: &SceneObjectHit, scene: &Scene, current_depth: u32) -> Color {
        let ref intersection = hit.hit.get_first_intersection();
        let check_size = 1f64 / self.checks_per_unit as f64;
        let color =
            if (intersection.uv.0 / check_size) as u32 % 2 == (intersection.uv.1 / check_size) as u32 % 2 {
                self.color_a
            } else {
                self.color_b
            };
        // TODO: How to do more properly??
        FlatMaterial {
            ambient: color * 0.1f64,
            diffuse: color,
            specular: SpecularLighting(BLACK, 0f64),
        }.get_color(ray, hit, scene, current_depth)
    }
}

#[derive(Debug)]
struct WeightedMaterial {
    weight: f64,
    material: Rc<Material>,
}

#[derive(Debug)]
pub struct MaterialComposition {
    materials: Vec<WeightedMaterial>,
}

impl MaterialComposition {
    pub fn new() -> MaterialComposition {
        MaterialComposition { materials: vec![] }
    }

    pub fn compose<M: Material + 'static>(mut self, material: &Rc<M>, weight: f64) -> MaterialComposition {
        if weight > 0f64 {
            self.materials.push(WeightedMaterial {
                material: Rc::<M>::clone(material),
                weight,
            });
        }
        self
    }
}

impl Material for MaterialComposition {
    fn get_color(&self, ray: &Ray, hit: &SceneObjectHit, scene: &Scene, current_depth: u32) -> Color {
        let ref intersection = hit.hit.get_first_intersection();
        let total_weight: f64 = (&self.materials)
            .iter()
            .map(|weighted_material| weighted_material.weight)
            .sum();
        let mut color = BLACK;
        for weighted_material in &self.materials {
            let contribution = weighted_material.weight / total_weight;
            color += weighted_material.material.get_color(ray, hit, scene, current_depth) * contribution;
        }

        color
    }
}
