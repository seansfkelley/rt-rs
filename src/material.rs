use objects::*;
use color::{ Color, BLACK, WHITE };
use std::path::Path;
use std::fmt::Debug;
use image;
use image::{ RgbImage, Pixel };

type SpecularExponent = f64;
pub struct SpecularLighting(pub Color, pub SpecularExponent);

type Reflectivity = f64;
pub struct ReflectiveLighting(pub Color, pub Reflectivity);

pub struct ComputedLighting {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: SpecularLighting,
    pub reflective: ReflectiveLighting,
}

pub trait Material: Debug {
    fn get_lighting(&self, intersection: &Intersection) -> ComputedLighting;
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
    fn get_lighting(&self, _intersection: &Intersection) -> ComputedLighting {
        ComputedLighting {
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: SpecularLighting(self.specular, self.specular_exponent),
            reflective: ReflectiveLighting(WHITE, self.reflectivity),
        }
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
    fn get_lighting(&self, intersection: &Intersection) -> ComputedLighting {
        let (width, height) = self.image.dimensions();
        let pixel = self.image.get_pixel((width as f64 * intersection.uv.0) as u32, (height as f64 * intersection.uv.1) as u32);
        let rgb = pixel.channels();
        let color = Color::new(rgb[0] as f64 / 255f64, rgb[1] as f64 / 255f64, rgb[2] as f64 / 255f64);

        // TODO: How to do more properly??
        ComputedLighting {
            ambient: color * 0.1f64,
            diffuse: color * 0.5f64,
            specular: SpecularLighting(BLACK, 0f64),
            reflective: ReflectiveLighting(color, self.reflectivity),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CheckerboardMaterial {
    pub checks_per_unit: u32,
    pub color_a: Color,
    pub color_b: Color,
}

impl Material for CheckerboardMaterial {
    fn get_lighting(&self, intersection: &Intersection) -> ComputedLighting {
        let check_size = 1f64 / self.checks_per_unit as f64;
        let color =
            if (intersection.uv.0 / check_size) as u32 % 2 == (intersection.uv.1 / check_size) as u32 % 2 {
                self.color_a
            } else {
                self.color_b
            };
        // TODO: How to do more properly??
        ComputedLighting {
            ambient: color * 0.1f64,
            diffuse: color * 0.5f64,
            specular: SpecularLighting(BLACK, 0f64),
            reflective: ReflectiveLighting(BLACK, 0f64),
        }
    }
}
