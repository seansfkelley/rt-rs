#[macro_use]
extern crate log;
extern crate env_logger;
extern crate image;

mod vector;
mod objects;
mod color;
mod scene;

use color::Color;
use vector::Vec3;
use scene::Scene;
use image::{RgbImage, Rgb, Pixel};
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();

    let camera_position = Vec3::new(0f64, 0f64, 20f64);
    let camera_up = Vec3::new(0f64, 1f64, 0f64).as_unit_vector();
    let camera_direction = Vec3::new(0f64, 0f64, -1f64).as_unit_vector();
    let camera_right = camera_direction.cross(camera_up).as_unit_vector();

    // TODO: Replace with proper FoV calculations.
    let pixel_grid_distance = 5f64;
    let pixel_grid_width = 5f64;
    let pixel_grid_height = 5f64;

    let width = 512u32;
    let height = 512u32;

    // TODO: Offset by half a unit so the ray is going through the center of the grid space.
    let x_step = camera_right * pixel_grid_width / width as f64;
    let y_step = -camera_up * pixel_grid_height / height as f64;
    let grid_center = camera_position + camera_direction * pixel_grid_distance;
    let grid_start = grid_center - x_step * (width as f64 / 2f64) - y_step * (height as f64 / 2f64);


    let sphere = objects::Sphere::new(Vec3::new(0f64, 0f64, 0f64), 5f64, Color::new(1f64, 0f64, 0f64));
    let light = objects::Light::new(Vec3::new(3f64, 6f64, 5f64), Color::new(1f64, 1f64, 1f64));
    let scene = Scene::new(vec![&sphere], vec![&light]);
    let background_color = Color::new(0f64, 0f64, 0f64);

    let mut img = RgbImage::new(width, height);

    // TODO: Antialiasing.
    for x in 0..width {
        for y in 0..height {
            // TODO: Scalar multiplication for non-floats?
            let origin = grid_start + x_step * x as f64 + y_step * y as f64;
            let direction = (origin - camera_position).as_unit_vector();
            let ray = objects::Ray::new(origin, direction);

            let closest = scene.cast_ray(ray);

            let color = match closest {
                Some(intersection) => { intersection.material },
                None => { background_color }
            };

            img.put_pixel(x, y, *Rgb::from_slice(&color.as_bytes()));
        }
    }

    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    image::ImageRgb8(img).save(fout, image::PNG).unwrap();
}
