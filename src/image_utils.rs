use std::path::Path;
use image::{ RgbImage, open as openImage };

pub fn load_image(path: &Path) -> RgbImage {
    match openImage(path) {
        Ok(img) => { img.to_rgb() }
        Err(reason) => { panic!("could not open image at {:?}: {:?}", path, reason); }
    }
}
