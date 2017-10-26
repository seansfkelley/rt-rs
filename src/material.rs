use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub specular_exponent: f64,
    pub reflectivity: f64,
}

pub mod plastic {
    use color::Color;
    use material::Material;

    pub fn create(color: Color) -> Material {
        return Material {
            ambient: color * 0.1,
            diffuse: color,
            specular: color * 0.5,
            specular_exponent: 5f64,
            reflectivity: 0.1,
        }
    }
}
