use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub specular_exponent: f64,
    pub reflectivity: f64,
}

pub fn plastic(color: Color) -> Material {
    return Material {
        ambient: color * 0.1,
        diffuse: color,
        specular: color * 0.5,
        specular_exponent: 2f64,
        reflectivity: 0.1,
    }
}

pub const MIRROR: Material = Material {
    ambient: Color { r: 0f64, g: 0f64, b: 0f64 },
    diffuse: Color { r: 0.05f64, g: 0.05f64, b: 0.05f64 },
    specular: Color { r: 1f64, g: 1f64, b: 1f64 },
    specular_exponent: 10f64,
    reflectivity: 0.95,
};
