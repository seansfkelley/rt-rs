use std::ops::{Add, Sub, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub const BLACK: Color = Color { r: 0f64, g: 0f64, b: 0f64 };

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        let clamped = self.clamp();
        [
            (clamped.r * 255f64) as u8,
            (clamped.g * 255f64) as u8,
            (clamped.b * 255f64) as u8,
        ]
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.max(0f64).min(1f64),
            g: self.g.max(0f64).min(1f64),
            b: self.b.max(0f64).min(1f64),
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, divisor: f64) -> Color {
        Color {
            r: self.r / divisor,
            g: self.g / divisor,
            b: self.b / divisor,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, multiplicand: f64) -> Color {
        Color {
            r: self.r * multiplicand,
            g: self.g * multiplicand,
            b: self.b * multiplicand,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}
