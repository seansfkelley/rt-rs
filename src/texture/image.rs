use image::{ RgbImage, Pixel };
use core::*;

impl Texture for RgbImage {
    fn get_color(&self, uv: Option<Uv>) -> Color {
        match uv {
            Some(Uv(u, v)) => {
                let (width, height) = self.dimensions();
                let x = (width as f64 * u) as u32;
                // rust-image has the origin at the top-left corner.
                let y = (height as f64 * (1f64 - v)) as u32;
                let pixel = self.get_pixel(x.min(width - 1), y.min(height - 1));
                let rgb = pixel.channels();
                Color::new(rgb[0] as f64 / 255f64, rgb[1] as f64 / 255f64, rgb[2] as f64 / 255f64)
            }
            None => { Color::BLACK }
        }
    }
}
