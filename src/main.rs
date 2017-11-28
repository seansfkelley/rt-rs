#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate rand;
extern crate regex;
extern crate lalrpop_util;
extern crate terminal_size;

mod core;
mod geometry;
mod scene;
mod util;
mod math;
mod importer;
mod progress_bar;

use core::*;
use rand::Rng;
use scene::Scene;
use image::{RgbImage, Rgb, Pixel};
use std::fs::File;
use std::path::Path;
use std::env;
use std::thread;
use std::time::Duration;
use std::sync::{ Arc, Mutex };
use std::io::{ stdout, Write };
use progress_bar::ProgressBar;

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

    let (width, height) = scene_file.parameters.image_dimensions;
    let antialias = scene_file.parameters.antialias;
    let mut camera = scene_file.camera;
    let frame_count = scene_file.animation.0;

    let progress_main = Arc::new(Mutex::new(ProgressBar::new(width * height * antialias * antialias * frame_count, frame_count)));
    let progress_render = progress_main.clone();
    thread::spawn(move || {
        loop {
            let is_complete = {
                let p = progress_render.lock().unwrap();
                p.render();
                p.is_complete()
            };

            if !is_complete {
                thread::sleep(Duration::from_millis(500));
            } else {
                progress_render.lock().unwrap().render();
                println!();
                stdout().flush().ok().unwrap();
                thread::sleep(Duration::from_millis(200)); // time to flush the buffers, sometimes
                break;
            }
        }
    });

    let mut rng = rand::thread_rng();
    for frame_number in 0..frame_count {
        progress_main.lock().unwrap().increment_frame();

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
                        color = color + scene.raytrace(&camera.get_ray(x as f64 + x_jitter, y as f64 + y_jitter));
                    }
                }
                progress_main.lock().unwrap().increment_operations(antialias * antialias);
                img.put_pixel(x, y, *Rgb::from_slice(&(color / (antialias * antialias) as f64).as_bytes()));
            }
        }

        let ref mut fout = File::create(&Path::new(&format!("out/{:03}.png", frame_number))).unwrap();
        image::ImageRgb8(img).save(fout, image::PNG).unwrap();

        camera = camera.transform(&scene_file.animation.1);
    }
}
