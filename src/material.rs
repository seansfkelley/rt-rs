use objects::*;
use color::Color;
use std::fmt::Debug;

pub trait Material: Debug {
    fn reflectivity(&self) -> f64;
    fn get_color(&self, ray: &Ray, intersection: &Intersection, lights: Vec<&Light>) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct PhongMaterial {
    ambient: Color,
    diffuse: Color,
    specular: Color,
    specular_exponent: f64,
    reflectivity: f64,
}

impl Material for PhongMaterial {
    fn reflectivity(&self) -> f64 { self.reflectivity }

    fn get_color(&self, ray: &Ray, intersection: &Intersection, lights: Vec<&Light>) -> Color {
        let mut color = self.ambient;
        for &light in &lights {
            let light_direction = (light.position - intersection.location).as_unit_vector();
            let diffuse_illumination = self.diffuse * light.color * intersection.normal.dot(light_direction).max(0f64);
            let specular_illumination = self.specular * light.color * intersection.normal.dot((light_direction - ray.direction).as_unit_vector()).max(0f64).powf(self.specular_exponent);
            color = color + diffuse_illumination + specular_illumination;
        }
        color
    }
}

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
