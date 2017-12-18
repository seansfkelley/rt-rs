use math::*;
use super::transform::*;
use super::ray::Ray;

// The image plane that pixels live on is infinitely large. This tuple defines what bounds, +/- from
// the origin, should be used as the screen. Increasing this while keeping the output image size constant
// will increase the "field of view", i.e., a larger physical area will be visible. With orthographic
// cameras, this is the only way to increase the "field of view". With perspective cameras, you should
// probably use the actual field_of_view value first, though you can also use this to tweak it.
pub type ScreenSize = (f64, f64);

#[derive(Debug, Clone, Copy)]
pub enum CameraKind {
    Orthographic,
    Perspective,
}

#[derive(Debug)]
pub struct Camera {
    raster_to_camera: Transform,
    camera_to_world: Transform,
    kind: CameraKind,
}

impl Camera {
    pub fn orthographic(camera_to_world: Mat4, screen_size: Option<ScreenSize>, image_dimensions: (u32, u32)) -> Camera {
        Camera {
            // for orthographic, the projection is nop because the rays will already be perpendicular to the image plane on creation
            raster_to_camera: Transform::new(compute_raster_to_camera(screen_size, image_dimensions, IDENTITY_MATRIX)),
            camera_to_world: Transform::new(camera_to_world),
            kind: CameraKind::Orthographic,
        }
    }

    pub fn perspective(camera_to_world: Mat4, screen_size: Option<ScreenSize>, image_dimensions: (u32, u32), fov: f64) -> Camera {
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

        Camera {
            raster_to_camera: Transform::new(compute_raster_to_camera(screen_size, image_dimensions, projection)),
            camera_to_world: Transform::new(camera_to_world),
            kind: CameraKind::Perspective,
        }
    }

    pub fn get_ray(&self, image_x: f64, image_y: f64) -> Ray {
        let ray = match self.kind {
            CameraKind::Orthographic => Ray::half_infinite(
                Point::new(image_x, image_y, 0f64).transform(&self.raster_to_camera),
                Vec3::Z_AXIS,
            ),
            CameraKind::Perspective => Ray::half_infinite(
                Point::uniform(0f64),
                Point::new(image_x, image_y, 0f64).transform(&self.raster_to_camera).into_vector().into_normalized(),
            )
        };
        ray.transform(&self.camera_to_world)
    }
}

impl Transformable for Camera {
    fn transform(&self, transform: &Transform) -> Camera {
        Camera {
            raster_to_camera: self.raster_to_camera.clone(),
            // TODO: Unsure if this is the right order.
            camera_to_world: Transform::new(&transform.m * &self.camera_to_world.m),
            kind: self.kind,
        }
    }

    fn invert_transform(&self, transform: &Transform) -> Camera {
        Camera {
            raster_to_camera: self.raster_to_camera.clone(),
            // TODO: Unsure if this is the right order.
            camera_to_world: Transform::new(&transform.m_inverse * &self.camera_to_world.m_inverse),
            kind: self.kind,
        }
    }
}

// pbrt ch. 6
fn compute_raster_to_camera(screen_size: Option<ScreenSize>, image_dimensions: (u32, u32), camera_to_screen: Mat4) -> Mat4 {
    // raster space: 0, 0 -> image_x, image_y
    // ndc space: 0, 0, -> 1, 1 ("normalized device coordinates")
    // screen space: -x, -y -> +x, +y (image plane)
    let image_x = image_dimensions.0 as f64;
    let image_y = image_dimensions.1 as f64;
    let (screen_x, screen_y) = match screen_size {
        Some(s) => s,
        // Following pbrt, we default keep the narrower side +/- 1 and proportionally increase the wider side.
        None => if image_x > image_y {
            (image_x / image_y * 2f64, 2f64)
        } else {
            (2f64, image_y / image_x * 2f64)
        },
    };

    // following pbrt, let's break out the interesting steps because we might want to slip other things
    // in between later (e.g. depth of field)
    let screen_to_raster =
        // ndc to raster scaling
        Mat4::create_scale(Vec3::new(image_x, image_y, 1f64)) *
        // screen to ndc scaling
        // -y -> flip top-to-bottom because graphics
        Mat4::create_scale(Vec3::new(1f64 / screen_x, -1f64 / screen_y, 1f64)) *
        // screen to ndc translation (origin at top-left corner)
        Mat4::create_translation(Vec3::new(screen_x / 2f64, -screen_y / 2f64, 0f64));

    camera_to_screen.invert().unwrap() * screen_to_raster.invert().unwrap()
}
