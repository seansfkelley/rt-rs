use std::ops::{ Add, Sub, Div, Mul, AddAssign, SubAssign, DivAssign, MulAssign };
use std::fmt::{ Debug, Formatter, Result };
use std::f64::{ INFINITY, NEG_INFINITY };
use math::*;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Color = Color { r: 0f64, g: 0f64, b: 0f64 };
    pub const WHITE: Color = Color { r: 1f64, g: 1f64, b: 1f64 };

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
            r: self.r.clamp(0f64, 1f64),
            g: self.g.clamp(0f64, 1f64),
            b: self.b.clamp(0f64, 1f64),
        }
    }

    pub fn average(&self) -> f64 {
        (self.r + self.g + self.b) / 3f64
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

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
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

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
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

impl DivAssign for Color {
    fn div_assign(&mut self, other: Color) {
        self.r /= other.r;
        self.g /= other.g;
        self.b /= other.b;
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

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, multiplicand: f64) {
        self.r *= multiplicand;
        self.g *= multiplicand;
        self.b *= multiplicand;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, other: Color) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Color<{}, {}, {}>", self.r, self.g, self.b)
    }
}

pub fn min_vs_max(colors: &[Color]) -> f64 {
    let mut min_r = INFINITY;
    let mut max_r = NEG_INFINITY;
    let mut min_g = INFINITY;
    let mut max_g = NEG_INFINITY;
    let mut min_b = INFINITY;
    let mut max_b = NEG_INFINITY;
    for color in colors {
        min_r = min_r.min(color.r);
        max_r = max_r.max(color.r);
        min_g = min_g.min(color.g);
        max_g = max_g.max(color.g);
        min_b = min_b.min(color.b);
        max_b = max_b.max(color.b);
    }
    max_r - min_r + max_g - min_g + max_b - min_b
}
