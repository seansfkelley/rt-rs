#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate rand;
extern crate regex;
extern crate lalrpop_util;
extern crate terminal_size;
extern crate ordered_float;
extern crate rayon;
extern crate noise;

mod core;
mod external_crate_traits;
mod lights;
mod geometry;
mod math;
mod importer;
mod progress_bar;
mod material;
mod renderer;
mod tessellation;

use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};
use std::env;
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::Arc;
use std::io::{stderr, Write};

use rayon::prelude::*;
use image::{RgbImage, Rgb, Pixel};

use core::*;
use scene::Scene;
use renderer::Renderer;
use progress_bar::ProgressBar;

fn seconds_since(t: SystemTime) -> f64 {
    let duration = t.elapsed().unwrap();
    (duration.as_secs() as f64 * 1e9f64 + duration.subsec_nanos() as f64) / 1e9f64
}

macro_rules! log_timing {
    ($description:expr, $task:expr) => {
        {
            eprint!("{}", $description);
            stderr().flush().ok().unwrap();
            let start_time = SystemTime::now();
            let result = $task;
            eprintln!("done in {:.1}s", seconds_since(start_time));
            result
        }
    };
}

fn main() {
    let mut args = env::args();
    args.next(); // Skip executable name.

    let scene_file_path = args.next().expect("exactly one argument required, but got zero");

    if args.count() > 0 {
        panic!("more than one argument provided");
    }

    let scene_file = log_timing!(
        format!("parsing {} and building objects... ", scene_file_path),
        importer::parse(Path::new(&scene_file_path)));

    let object_tree = log_timing!(
        "building spatial index... ",
        KdTree::from(scene_file.objects));

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

    let (width, height) = scene_file.parameters.image_dimensions;
    let frame_count = scene_file.animation.0;

    let progress = Arc::new(ProgressBar::new((width * height * frame_count) as usize, frame_count as usize));
    let progress_render = Arc::clone(&progress);
    thread::spawn(move || {
        loop {
            let is_complete = {
                progress_render.render();
                progress_render.is_complete()
            };

            if !is_complete {
                thread::sleep(Duration::from_millis(100));
            } else {
                progress_render.render();
                eprintln!();
                stderr().flush().ok().unwrap();
                thread::sleep(Duration::from_millis(200)); // time to flush the buffers, sometimes
                break;
            }
        }
    });

    let mut moving_camera = scene_file.camera;
    let mut renderer = Renderer::new(
        Scene { objects: object_tree, lights: scene_file.lights },
        scene_file.parameters,
        moving_camera.clone());

    for frame_number in 0..frame_count {
        progress.increment_frame();

        let mut img = RgbImage::new(width, height);

        (0..width)
            .into_par_iter()
            .map(|image_x| {
                (0..height)
                    .map(|image_y| {
                        let color = renderer.render_pixel(image_x, image_y);
                        progress.increment_operations(1);
                        (image_x, image_y, color)
                    })
                    .collect::<Vec<(u32, u32, Color)>>()
            })
            .collect::<Vec<Vec<(u32, u32, Color)>>>()
            .into_iter()
            .flat_map(|v| v.into_iter())
            .for_each(|(x, y, color)| {
                img.put_pixel(x, y, *Rgb::from_slice(&color.as_bytes()));
            });


        let ref mut output_file = File::create(&get_output_filename(frame_number)).expect("error creating output file");
        image::ImageRgb8(img).save(output_file, image::PNG).expect("error saving image");

        moving_camera = moving_camera.transform(&scene_file.animation.1);
        renderer = renderer.with_camera(moving_camera.clone());
    }
}
