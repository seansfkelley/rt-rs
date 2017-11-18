use math::*;
use super::transform::Transform;
use super::ray::Ray;

#[derive(Debug, Clone)]
pub struct Camera {
    camera_to_screen: Transform,
}

impl Camera {
    pub fn new(position: Point, look_at: Point, up: Vec3, dimensions: (u32, u32)) -> Camera {
        // raster space: 0, 0 -> image_x, image_y
        // ndc space: 0, 0, -> 1, 1
        // screen space: viewing frustrum (this is where fov comes into play)
        // camera space: camera at the origin
        // screenwindow: [-x, +x, -y, +y]
        let (image_x, image_y) = (dimensions.0 as f64, dimensions.1 as f64);
        let aspect_ratio = image_x / image_y;
        let (screen_x, screen_y) = if aspect_ratio > 1f64 {
            (aspect_ratio, 1f64)
        } else {
            (1f64, 1f64 / aspect_ratio)
        };
        let screen_to_raster =
            // ndc to raster scaling
            Mat4::create_scale(Vec3::new(image_x, image_y, 1f64)) *
            // screen to ndc scaling
            Mat4::create_scale(Vec3::new(1f64 / (screen_x * 2f64), 1f64 / (screen_y * 2f64), 1f64)) *
            // -y -> flip top-to-bottom because graphics
            // screen (which is +/- x, +/- y) to ndc (which is 0, 0) translation
            Mat4::create_translation(Vec3::new(screen_x, -screen_y, 0f64));
        let camera_to_screen = ; // TODO: Perspective transformation.
        Camera {
            camera_to_screen: Transform::new()
        }
    }

    pub fn get_ray(image_x: usize, image_y: usize) -> Ray {

    }
}
