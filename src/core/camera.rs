use std::fmt::Debug;
use math::*;
use super::transform::*;
use super::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct BaseCamera {
    pub position: Point,
    pub look_at: Point,
    pub up: Vec3,
    pub screen_size: (f64, f64),
}

pub trait Camera : Debug {
    fn get_ray(&self, image_x: u32, image_y: u32) -> Ray;
}

#[derive(Debug, Clone, Copy)]
pub struct OrthographicCamera {
    base: BaseCamera,
    raster_to_world: Transform,
}

#[derive(Debug, Clone, Copy)]
pub struct PerspectiveCamera {
    base: BaseCamera,
    raster_to_world: Transform,
}

impl OrthographicCamera {
    pub fn new(base: BaseCamera, dimensions: (u32, u32)) -> OrthographicCamera {
        // for orthographic, the projection is nop because the rays will already be perpendicular to the image plane on creation
        OrthographicCamera {
            base,
            raster_to_world: Transform::new(compute_raster_to_world(base, dimensions, IDENTITY_MATRIX))
        }
    }
}

impl Camera for OrthographicCamera {
    fn get_ray(&self, image_x: u32, image_y: u32) -> Ray {
        Ray {
            origin: Point::new(image_x as f64, image_y as f64, 0f64).transform(&self.raster_to_world),
            direction: Vec3::new(0f64, 0f64, 1f64).transform(&self.raster_to_world).as_normalized(),
        }
    }
}

impl PerspectiveCamera {
    pub fn new(base: BaseCamera, dimensions: (u32, u32), fov: f64) -> PerspectiveCamera {
        // pbrt pg. 311
        // TODO: pbrt says these can be arbitrary. Why? Can we just use 0 and 1?
        let far = 1000f64;
        let near = 0.01f64;
        let inverse_tan = 1f64 / (fov.to_radians() / 2f64).tan();
        let projection = Mat4::create_scale(Vec3::new(inverse_tan, inverse_tan, 1f64)) * Mat4 {
            cells: [
                [1f64, 0f64,               0f64,                       0f64],
                [0f64, 1f64,               0f64,                       0f64],
                [0f64, 0f64, far / (far - near), -far * near / (far - near)],
                [0f64, 0f64,               1f64,                       0f64],
            ],
        };

        PerspectiveCamera {
            base,
            raster_to_world: Transform::new(compute_raster_to_world(base, dimensions, projection))
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, image_x: u32, image_y: u32) -> Ray {
        Ray {
            // TODO: Want to be able to access camera -> world because we should be transforming 0, 0, 0 (origin) in _camera space_, not raster space, to world space.
            origin: Point::uniform(0f64).transform(&self.raster_to_world),
            // TODO: Didn't actually change this one from orthographic.
            direction: Vec3::new(0f64, 0f64, 1f64).transform(&self.raster_to_world).as_normalized(),
        }
    }
}

// pbrt ch. 6
fn compute_raster_to_world(base: BaseCamera, dimensions: (u32, u32), camera_to_screen: Mat4) -> Mat4 {
    let world_to_camera = Mat4::create_look_at(base.position, base.look_at, base.up);

    // raster space: 0, 0 -> image_x, image_y
    // ndc space: 0, 0, -> 1, 1 ("normalized device coordinates")
    // screen space: -x, -y -> +x, +y (image plane)
    let (image_x, image_y) = dimensions;
    let (screen_x, screen_y) = base.screen_size;

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


    world_to_camera.invert().unwrap() * camera_to_screen.invert().unwrap() * screen_to_raster.invert().unwrap()
}
