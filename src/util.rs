use core::intersection::{Uv};
use core::color::{Color};
use image::{ RgbImage, Pixel };

pub trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Clamp for f64 {
    fn clamp(self, min: f64, max: f64) -> f64 {
        self.min(max).max(min)
    }
}

pub trait UvMap {
    fn get_color(&self, uv: Uv) -> Color;
}

impl UvMap for RgbImage {
    fn get_color(&self, uv: Uv) -> Color {
        let (width, height) = self.dimensions();
        let x = (width as f64 * uv.0) as u32;
        // rust-image has the origin at the top-left corner.
        let y = (height as f64 * (1f64 - uv.1)) as u32;
        let pixel = self.get_pixel(x.min(width - 1), y.min(height - 1));
        let rgb = pixel.channels();
        Color::new(rgb[0] as f64 / 255f64, rgb[1] as f64 / 255f64, rgb[2] as f64 / 255f64)
    }
}
