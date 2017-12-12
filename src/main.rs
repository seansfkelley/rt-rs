#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate rand;
extern crate regex;
extern crate lalrpop_util;
extern crate terminal_size;
extern crate ordered_float;
extern crate rayon;

mod core;
mod geometry;
mod scene;
mod util;
mod math;
mod importer;
mod progress_bar;

use std::fs::{ File, create_dir_all };
use std::path::{ Path, PathBuf };
use std::collections::HashMap;
use std::iter::FromIterator;
use std::env;
use std::thread;
use std::time::{ Duration, SystemTime };
use std::sync::{ Arc, Mutex };
use std::io::{ stderr, Write };

use rayon::prelude::*;
use image::{ RgbImage, Rgb, Pixel };

use core::*;
use rand::Rng;
use scene::Scene;
use progress_bar::ProgressBar;

fn seconds_since(t: SystemTime) -> f64 {
    let duration = t.elapsed().unwrap();
    (duration.as_secs() as f64 * 1e9f64 + duration.subsec_nanos() as f64) / 1e9f64
}

fn main() {
    let mut args = env::args();
    args.next(); // Skip executable name.

    let scene_file_path = args.next().expect("exactly one argument required, but got zero");

    if args.count() > 0 {
        panic!("more than one argument provided");
    }

    let parse_start_time = SystemTime::now();
    let scene_file = importer::parse(Path::new(&scene_file_path));
    eprintln!("{} parsed and objects constructed in {:.1}s", scene_file_path, seconds_since(parse_start_time));

    let tree_start_time = SystemTime::now();
    let object_tree = KdTree::from(scene_file.objects);
    eprintln!("spatial index built in {:.1}s", seconds_since(tree_start_time));

    let output_directory: PathBuf = vec![
        "out",
        Path::new(&scene_file_path).file_stem().expect("no file stem").to_str().expect("cannot convert path to string"),
    ].iter().collect();
    create_dir_all(&output_directory).expect("could not create output directory");

    let get_output_filename = move |i: u32| -> Box<Path> {
        let mut p = output_directory.clone();
        p.push(Path::new(&format!("{:03}.png", i)));
        p.into_boxed_path()
    };

    let scene = Scene::new(
        object_tree,
        scene_file.lights,
        scene_file.parameters.background_color,
        scene_file.parameters.depth_limit
    );

    let (width, height) = scene_file.parameters.image_dimensions;
    let antialias = scene_file.parameters.antialias;
    let mut camera = scene_file.camera;
    let frame_count = scene_file.animation.0;

    // let progress_main = Arc::new(Mutex::new(ProgressBar::new(width * height * antialias * antialias * frame_count, frame_count)));
    // let progress_render = progress_main.clone();
    // thread::spawn(move || {
    //     loop {
    //         let is_complete = {
    //             let p = progress_render.lock().unwrap();
    //             p.render();
    //             p.is_complete()
    //         };

    //         if !is_complete {
    //             thread::sleep(Duration::from_millis(100));
    //         } else {
    //             progress_render.lock().unwrap().render();
    //             eprintln!();
    //             stderr().flush().ok().unwrap();
    //             thread::sleep(Duration::from_millis(200)); // time to flush the buffers, sometimes
    //             break;
    //         }
    //     }
    // });

    for frame_number in 0..frame_count {
        // progress_main.lock().unwrap().increment_frame();

        let mut img = RgbImage::new(width, height);

        (0..width)
            .flat_map(|x| (0..height).map(|y| (x, y)).collect::<Vec<(u32, u32)>>().into_iter())
            .collect::<Vec<(u32, u32)>>()
            .into_par_iter()
            .map(|(image_x, image_y)| {
                let mut rng = rand::thread_rng();

                let mut color = color::BLACK.clone();
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
                        color = color + scene.raytrace(camera.get_ray(image_x as f64 + x_jitter, image_y as f64 + y_jitter));
                    }
                }
                (image_x, image_y, color / (antialias * antialias) as f64)
            })
            .collect::<Vec<(u32, u32, Color)>>()
            .into_iter()
            .for_each(|(x, y, color)| {
                img.put_pixel(x, y, *Rgb::from_slice(&color.as_bytes()));
            });

        // progress_main.lock().unwrap().increment_operations(antialias * antialias);

        let ref mut output_file = File::create(&get_output_filename(frame_number)).expect("error creating output file");
        image::ImageRgb8(img).save(output_file, image::PNG).expect("error saving image");

        camera = camera.transform(&scene_file.animation.1);
    }
}
