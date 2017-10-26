extern crate image;
extern crate rand;

mod vector;
mod objects;
mod color;
mod scene;
mod material;

use rand::Rng;
use color::Color;
use vector::Vec3;
use scene::Scene;
use image::{RgbImage, Rgb, Pixel};
use std::fs::File;
use std::path::Path;
use material::plastic;

fn main() {
    let camera_position = Vec3::new(0f64, 0f64, 25f64);
    let camera_up = Vec3::new(0f64, 1f64, 0f64).as_unit_vector();
    let camera_direction = Vec3::new(0f64, 0f64, -1f64).as_unit_vector();
    let camera_right = camera_direction.cross(camera_up).as_unit_vector();

    // TODO: Replace with proper FoV calculations.
    let pixel_grid_distance = 5f64;
    let pixel_grid_width = 5f64;
    let pixel_grid_height = 5f64;

    let width = 512u32;
    let height = 512u32;
    let antialias = 2;

    let x_step = camera_right * pixel_grid_width / width as f64;
    let y_step = -camera_up * pixel_grid_height / height as f64;
    let grid_center = camera_position + camera_direction * pixel_grid_distance;
    let grid_start = grid_center - x_step * (width as f64 / 2f64) - y_step * (height as f64 / 2f64);

    let red_plastic = plastic::create(Color::new(0f64, 0.7f64, 0.7f64));
    let sphere1 = objects::Sphere::new(Vec3::new(-4f64, -4f64, 0f64), 5f64, red_plastic);
    let sphere2 = objects::Sphere::new(Vec3::new(4f64, 4f64, 0f64), 5f64, red_plastic);
    let light1 = objects::Light::new(Vec3::new(5f64, 5f64, 10f64), Color::new(0.4f64, 0.4f64, 0.4f64));
    let light2 = objects::Light::new(Vec3::new(-10f64, -10f64, 7f64), Color::new(0.4f64, 0.4f64, 0.4f64));
    let scene = Scene::new(vec![&sphere1, &sphere2], vec![&light1, &light2], Color::new(0f64, 0f64, 0f64), 3);

    let mut rng = rand::thread_rng();
    let mut img = RgbImage::new(width, height);

    // TODO: Antialiasing.
    for x in 0..width {
        for y in 0..height {
            for sample_x in 0..antialias {
                for sample_y in 0..antialias {
                    let (x_min, x_max, y_min, y_max) = (
                        sample_x as f64 / antialias as f64,
                        1f64 + sample_x as f64 / antialias as f64,
                        sample_y as f64 / antialias as f64,
                        1f64 + sample_y as f64 / antialias as f64
                    );

                    let (x_jitter, y_jitter) = (
                        rng.next_f64() * (x_max - x_min) + x_min,
                        rng.next_f64() * (y_max - y_min) + y_min,
                    );

                    println!("{} {}", x_jitter, y_jitter);

                    // TODO: Scalar multiplication for non-floats?
                    let origin = grid_start + x_step * (x as f64 - x_jitter) + y_step * (y as f64 - y_jitter);
                    let direction = (origin - camera_position).as_unit_vector();
                    let ray = objects::Ray::new(origin, direction);
                    img.put_pixel(x, y, *Rgb::from_slice(&scene.raytrace(ray).as_bytes()));
                }
            }
        }
    }

    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    image::ImageRgb8(img).save(fout, image::PNG).unwrap();
}
