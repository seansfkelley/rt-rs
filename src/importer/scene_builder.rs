use std::collections::HashMap;
use std::boxed::Box;
use std::sync::Arc;
use core::*;
use math::*;
use material::*;

#[derive(Default)]
pub struct SceneBuilder {
    camera: Option<CameraBuilder>,
    animation: Option<(u32, Vec<Mat4>)>,
    image_dimensions: Option<(u32, u32)>,
    antialias: Option<u32>,
    antialias_tolerance: Option<f64>,
    depth_limit: Option<u32>,
    background_color: Option<Color>,
    textures: HashMap<String, Arc<Texture>>,
    // TODO: Should transform be an Arc instead? Feels like this can get expensive.
    transform_stack: Vec<Transform>,
    pub objects: Vec<SceneObject>,
    pub lights: Vec<Box<LightType>>,
}

pub type CameraCommon = (Point, Point, Vec3, Option<ScreenSize>);

#[derive(Debug, Clone, Copy)]
pub enum CameraBuilder {
    Orthographic(CameraCommon),
    Perspective(CameraCommon, f64)
}

fn camera_to_world(common: CameraCommon) -> Mat4 {
    Mat4::create_look_at(common.0, common.1, common.2).invert().unwrap()
}

macro_rules! optional_setter {
    ($name:ident, $type:ty) => {
        pub fn $name(&mut self, input: $type) {
            if self.$name.is_some() {
                eprintln!("warning: scene file overrode already-set value for \"{}\"", stringify!($name));
            }
            self.$name = Some(input);
        }
    };
}

macro_rules! require_optional {
    ($self_:ident, $name:ident) => {
        if $self_.$name.is_some() {
            $self_.$name.unwrap()
        } else {
            panic!("scene file didn't set \"{}\" property", stringify!($name))
        }
    }
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        Default::default()
    }

    optional_setter!(camera, CameraBuilder);
    optional_setter!(animation, (u32, Vec<Mat4>));
    optional_setter!(image_dimensions, (u32, u32));
    optional_setter!(antialias, u32);
    optional_setter!(antialias_tolerance, f64);
    optional_setter!(depth_limit, u32);
    optional_setter!(background_color, Color);

    pub fn register_texture(&mut self, name: &str, texture: Box<Texture>) {
        self.textures.insert(name.to_owned(), Arc::from(texture));
    }

    fn get_current_transform(&self) -> Transform {
        match self.transform_stack.last() {
            Some(transform) => transform.clone(),
            None => IDENTITY_TRANSFORM.clone(),
        }
    }

    pub fn push_transform(&mut self, mat: Mat4) {
        let new_transform_matrix = mat * self.get_current_transform().m;
        self.transform_stack.push(Transform::new(new_transform_matrix));
    }

    pub fn pop_n_transforms(&mut self, count: u32) {
        for _ in 0..count {
            match self.transform_stack.pop() {
                Some(_) => {},
                None => { panic!("tried to pop an empty transform stack"); },
            };
        }
    }

    pub fn pop_all_transforms(&mut self) {
        let count = self.transform_stack.len() as u32;
        self.pop_n_transforms(count);
    }

    pub fn add_object(&mut self, partial_object: (Box<Geometry>, &str)) {
        let transform = self.get_current_transform();
        let texture_name = partial_object.1.to_owned();
        self.objects.push(SceneObject {
            shape: Shape::new(Arc::from(partial_object.0), transform),
            texture: Arc::clone(self.textures.get(&texture_name).expect(format!("no texture named \"{}\" defined", texture_name).as_str())),
        });
    }

    pub fn add_light(&mut self, light: Box<LightType>) {
        self.lights.push(light);
    }

    pub fn build_camera(&self) -> Camera {
        let dimensions = require_optional!(self, image_dimensions);
        match require_optional!(self, camera) {
            CameraBuilder::Orthographic(common) => {
                Camera::orthographic(camera_to_world(common), common.3, dimensions)
            }
            CameraBuilder::Perspective(common, fov) => {
                Camera::perspective(camera_to_world(common), common.3, dimensions, fov)
            }
        }
    }

    pub fn build_render_parameters(&self) -> RenderParamaters {
        RenderParamaters {
            image_dimensions: require_optional!(self, image_dimensions),
            antialias: self.antialias.unwrap_or(1),
            antialias_tolerance: self.antialias_tolerance.unwrap_or(0.01f64),
            depth_limit: self.depth_limit.unwrap_or(3),
            background_color: self.background_color.unwrap_or(Color::BLACK),
        }
    }

    pub fn build_animation(&self) -> (u32, Transform) {
        match self.animation {
            Some((ref frames, ref matrices)) => (
                *frames,
                Transform::new(matrices.into_iter().rev().fold(IDENTITY_MATRIX, |a, b| a * b)),
            ),
            None => (1, Transform::new(IDENTITY_MATRIX)),
        }
    }
}
