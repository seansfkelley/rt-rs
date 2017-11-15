#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate rand;
extern crate regex;
extern crate lalrpop_util;

mod core;
mod geometry;
mod scene;
mod util;
mod test;
mod math;
mod importer;

use core::*;
use math::*;
use rand::Rng;
use scene::Scene;
use image::{RgbImage, Rgb, Pixel};
use std::fs::File;
use std::path::Path;
use std::f64::consts::PI;
use std::env;

fn main() {
    let mut args = env::args();
    args.next(); // Skip executable name.

    let scene_file_path = args.next().expect("exactly one argument required, but got zero");

    if args.count() > 0 {
        panic!("more than one argument provided");
    }

    let scene_file = importer::parse(Path::new(&scene_file_path));

    let scene = Scene::new(
        scene_file.objects,
        scene_file.lights,
        scene_file.parameters.background_color,
        scene_file.parameters.depth_limit
    );

    // TODO: Replace with proper FoV calculations.
    // TODO: Put into the file.
    let pixel_grid_distance = 5f64;
    let pixel_grid_width = 5f64;
    let pixel_grid_height = 5f64;

    let (width, height) = scene_file.parameters.image_dimensions;
    let antialias = scene_file.parameters.antialias;

    let mut camera = scene_file.camera.clone();

    // TODO: Put into the file!
    let frames = 1u32;
    let ref rotation = Transform::new(Mat4::create_rotation((2f64 * PI) / frames as f64, Y_AXIS));

    for i in 0..frames {
        let x_step = camera.right * pixel_grid_width / width as f64;
        let y_step = -camera.up * pixel_grid_height / height as f64;
        let grid_center = camera.position + camera.direction * pixel_grid_distance;
        let grid_start = grid_center - x_step * (width as f64 / 2f64) - y_step * (height as f64 / 2f64);

        let mut rng = rand::thread_rng();
        let mut img = RgbImage::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let mut color = color::BLACK;
                for sample_x in 0..antialias {
                    for sample_y in 0..antialias {
                        let (x_jitter, y_jitter) =
                            if antialias == 1 {
                                (0.5f64, 0.5f64)
                            } else {
                                let (x_min, x_max, y_min, y_max) = (
                                    sample_x as f64 / antialias as f64,
                                    (1f64 + sample_x as f64) / antialias as f64,
                                    sample_y as f64 / antialias as f64,
                                    (1f64 + sample_y as f64) / antialias as f64
                                );

                                (
                                    rng.next_f64() * (x_max - x_min) + x_min,
                                    rng.next_f64() * (y_max - y_min) + y_min,
                                )
                            };

                        let origin = grid_start + x_step * (x as f64 + x_jitter) + y_step * (y as f64 + y_jitter);
                        let direction = (origin - camera.position).as_normalized();
                        let ray = Ray::new(origin, direction);
                        color = color + scene.raytrace(ray);
                    }
                }
                img.put_pixel(x, y, *Rgb::from_slice(&(color / (antialias * antialias) as f64).as_bytes()));
            }
        }

        let ref mut fout = File::create(&Path::new(&format!("out/{:03}.png", i))).unwrap();
        image::ImageRgb8(img).save(fout, image::PNG).unwrap();

        camera = camera.transform(rotation);
    }
}
