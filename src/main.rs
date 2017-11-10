#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate rand;
extern crate regex;

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

fn main() {
    let scene_file = importer::parse(Path::new("scenes/basic.scene"));

    println!("{:?}", scene_file);

    // let cyan_plastic: Rc<material::Material> = Rc::new(material::FlatMaterial { color: Color::new(0f64, 0.7f64, 0.7f64), specular_exponent: 1f64, reflectivity: 0.1f64 });
    // let bw_checkerboard: Rc<material::Material> = Rc::new(material::CheckerboardMaterial { checks_per_unit: 32, color_a: BLACK, color_b: WHITE });
    // let mirror: Rc<material::Material> = Rc::new(material::FlatMaterial { color: Color::new(0.9f64, 0.9f64, 0.9f64), specular_exponent: 7f64, reflectivity: 0.9f64 });
    // let yellow_matte: Rc<material::Material> = Rc::new(material::FlatMaterial { color: Color::new(0.7f64, 0.7f64, 0f64), specular_exponent: 0f64, reflectivity: 0f64 });

    // let yellow_sphere_transform = Transform::new(Mat4::create_translation(Vec3::new(4f64, -4f64, 0f64)));
    // let ref unit_sphere = Rc::new(Sphere::new(1f64));
    // let ref three_sphere = Rc::new(Sphere::new(3f64));
    // let ref five_sphere = Rc::new(Sphere::new(5f64));
    // let bite_transform = Transform::new(Mat4::create_translation(Vec3::new(3f64, -3.5f64, 0.5f64)));
    // let bite_positive = Rc::new(Shape::new(three_sphere, yellow_sphere_transform));
    // let bite_negative = Rc::new(SceneObject::new(three_sphere, bite_transform, &yellow_matte));
    // let bite = Rc::new(Difference::new(bite_positive, bite_negative));

    // let scene_objects: Vec<SceneObject> = vec![
    //     SceneObject::new(unit_sphere, Transform::new(Mat4::create_translation(Vec3::new(-4f64, -4f64, 2f64))), &cyan_plastic),
    //     SceneObject::new(five_sphere, Transform::new(Mat4::create_translation(Vec3::new(4f64, 4f64, 0f64))), &mirror),
    //     SceneObject::new(three_sphere, Transform::new(Mat4::create_translation(Vec3::new(-5f64, 4f64, 0f64))), &bw_checkerboard),
    //     SceneObject::from_geo(&bite, &yellow_matte),
    //     SceneObject::from_geo(&triangle_mesh_geo, &cyan_plastic),
    // ];

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

        let ref mut fout = File::create(&Path::new(&format!("out/{:02}.png", i))).unwrap();
        image::ImageRgb8(img).save(fout, image::PNG).unwrap();

        camera = camera.transform(rotation);
    }
}
