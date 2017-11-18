use math::*;
use super::transform::*;
use super::ray::Ray;

#[derive(Debug, Clone)]
pub struct Camera {
    raster_to_world: Transform,
}

impl Camera {
    // pbrt ch. 6
    fn new(position: Point, look_at: Point, up: Vec3, screen_size: (f64, f64), dimensions: (u32, u32), camera_to_screen: Mat4) -> Camera {
        let world_to_camera = Mat4::create_look_at(position, look_at, up);

        // raster space: 0, 0 -> image_x, image_y
        // ndc space: 0, 0, -> 1, 1 ("normalized device coordinates")
        // screen space: -x, -y -> +x, +y (image plane)
        let (image_x, image_y) = dimensions;
        let (screen_x, screen_y) = screen_size;

        // following pbrt, let's break out the interesting steps because we might want to slip other things
        // in between later (e.g. depth of field)
        let screen_to_raster =
            // ndc to raster scaling
            Mat4::create_scale(Vec3::new(image_x as f64, image_y as f64, 1f64)) *
            // screen to ndc scaling
            // -y -> flip top-to-bottom because graphics
            Mat4::create_scale(Vec3::new(1f64 / screen_x, -1f64 / screen_y, 1f64)) *
            // screen to ndc translation (origin at top-left corner)
            Mat4::create_translation(Vec3::new(screen_x / 2f64, -screen_y / 2f64, 0f64));

        Camera {
            raster_to_world: Transform::new(
                world_to_camera.invert().unwrap() *
                camera_to_screen.invert().unwrap() *
                screen_to_raster.invert().unwrap())
        }
    }

    pub fn orthographic(position: Point, look_at: Point, up: Vec3, screen_size: (f64, f64), dimensions: (u32, u32)) -> Camera {
        // for orthographic, the projection is nop because the rays will already be perpendicular to the image plane on creation
        Camera::new(position, look_at, up, screen_size, dimensions, IDENTITY_MATRIX)
    }

    pub fn perspective(position: Point, look_at: Point, up: Vec3, screen_size: (f64, f64), fov: f64, dimensions: (u32, u32)) -> Camera {
        panic!("perspective cameras are not implemented yet");
    }

    pub fn get_ray(&self, image_x: u32, image_y: u32) -> Ray {
        Ray {
            origin: Point::new(image_x as f64, image_y as f64, 0f64).transform(&self.raster_to_world),
            direction: Vec3::new(0f64, 0f64, 1f64).transform(&self.raster_to_world).as_normalized(),
        }
    }
}
