#[macro_use]
extern crate log;
extern crate env_logger;
extern crate image;

mod vector;
mod objects;

use vector::Vec3;
use image::{RgbImage, Rgb, Pixel};
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();

    let mut img = RgbImage::new(512, 512);
    img.put_pixel(20, 20, Rgb::from_channels(255, 0, 0, 0));
    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageRgb8(img).save(fout, image::PNG);
}
