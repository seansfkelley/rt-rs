#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate image;
extern crate rand;

mod core;
mod geometry;
mod scene;
mod material;
mod util;
mod test;
mod math;

use core::*;
use geometry::*;
use math::*;
use rand::Rng;
use scene::Scene;
use image::{RgbImage, Rgb, Pixel};
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

fn main() {
    let camera_position = Point::new(0f64, 0f64, 25f64);
    let camera_up = Vec3::new(0f64, 1f64, 0f64).as_normalized();
    let camera_direction = Vec3::new(0f64, 0f64, -1f64).as_normalized();
    let camera_right = camera_direction.cross(camera_up).as_normalized();

    // TODO: Replace with proper FoV calculations.
    let pixel_grid_distance = 5f64;
    let pixel_grid_width = 5f64;
    let pixel_grid_height = 5f64;

    let width = 512u32;
    let height = 512u32;
    let antialias = 2u32;

    let x_step = camera_right * pixel_grid_width / width as f64;
    let y_step = -camera_up * pixel_grid_height / height as f64;
    let grid_center = camera_position + camera_direction * pixel_grid_distance;
    let grid_start = grid_center - x_step * (width as f64 / 2f64) - y_step * (height as f64 / 2f64);

    let cyan_plastic: Rc<material::Material> = Rc::new(material::FlatMaterial { color: Color::new(0f64, 0.7f64, 0.7f64), specular_exponent: 1f64, reflectivity: 0.1f64 });
    let bw_checkerboard: Rc<material::Material> = Rc::new(material::CheckerboardMaterial { checks_per_unit: 32, color_a: BLACK, color_b: WHITE });
    let mirror: Rc<material::Material> = Rc::new(material::FlatMaterial { color: Color::new(0.9f64, 0.9f64, 0.9f64), specular_exponent: 7f64, reflectivity: 0.9f64 });
    let yellow_matte: Rc<material::Material> = Rc::new(material::FlatMaterial { color: Color::new(0.7f64, 0.7f64, 0f64), specular_exponent: 0f64, reflectivity: 0f64 });

    let yellow_sphere_transform = Transform::new(Mat4::create_translation(Vec3::new(4f64, -4f64, 0f64)));
    let ref unit_sphere = Rc::new(Sphere::new(1f64));
    let ref three_sphere = Rc::new(Sphere::new(3f64));
    let ref five_sphere = Rc::new(Sphere::new(5f64));
    let bite_transform = Transform::new(Mat4::create_translation(Vec3::new(3f64, -3.5f64, 0.5f64)));
    let bite_positive = Rc::new(Shape::new(three_sphere, yellow_sphere_transform));
    let bite_negative = Rc::new(SceneObject::new(three_sphere, bite_transform, &yellow_matte));
    let bite = Rc::new(Difference::new(bite_positive, bite_negative));
    let triangle_mesh_geo = Rc::new(TriangleMesh::new(
        vec![
            Point::new(-3f64, -3f64, 0f64),
            Point::new(3f64, -3f64, 0f64),
            Point::new(0f64, 3f64, 0f64),
        ],
        vec![], vec![], vec![(0, 1, 2)],
    ));

    let scene_objects: Vec<SceneObject> = vec![
        SceneObject::new(unit_sphere, Transform::new(Mat4::create_translation(Vec3::new(-4f64, -4f64, 2f64))), &cyan_plastic),
        SceneObject::new(five_sphere, Transform::new(Mat4::create_translation(Vec3::new(4f64, 4f64, 0f64))), &mirror),
        SceneObject::new(three_sphere, Transform::new(Mat4::create_translation(Vec3::new(-5f64, 4f64, 0f64))), &bw_checkerboard),
        SceneObject::from_geo(&bite, &yellow_matte),
        SceneObject::from_geo(&triangle_mesh_geo, &cyan_plastic),
    ];

    let scene_lights: Vec<Box<Light>> = vec![
        Box::new(Light::new(Point::new(5f64, 5f64, 10f64), Color::new(0.4f64, 0.4f64, 0.4f64))),
        Box::new(Light::new(Point::new(-15f64, -15f64, 0f64), Color::new(0.4f64, 0.4f64, 0.4f64))),
    ];

    let scene = Scene::new(scene_objects, scene_lights, Color::new(0.1f64, 0.1f64, 0.1f64), 0);

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
                    let direction = (origin - camera_position).as_normalized();
                    let ray = Ray::new(origin, direction);
                    color = color + scene.raytrace(ray);
                }
            }
            img.put_pixel(x, y, *Rgb::from_slice(&(color / (antialias * antialias) as f64).as_bytes()));
        }
    }

    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    image::ImageRgb8(img).save(fout, image::PNG).unwrap();
}
