use std::collections::HashMap;
use std::boxed::Box;
use core::*;
use math::*;

#[derive(Debug, Default)]
pub struct SceneBuilder {
    camera_position: Option<Point>,
    camera_up: Option<Vec3>,
    camera_look_at: Option<Point>,
    antialias: Option<u32>,
    pub materials: HashMap<String, Box<Material>>,
    transform_stack: Vec<Mat4>,
}

macro_rules! optional_setter {
    ($name:ident, $type:ty) => {
        pub fn $name(&mut self, input: $type) {
            self.$name = Some(input);
        }
    };
}

macro_rules! require_optional {
    ($self_:ident, $name:ident) => {
        if $self_.$name.is_some() {
            $self_.$name.unwrap()
        } else {
            panic!("scene file didn't set '{}' property", stringify!($name))
        }
    }
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        Default::default()
    }

    optional_setter!(camera_position, Point);
    optional_setter!(camera_up, Vec3);
    optional_setter!(camera_look_at, Point);
    optional_setter!(antialias, u32);

    pub fn register_material(&mut self, name: &str, material: Box<Material>) {
        self.materials.insert(name.to_owned(), material);
    }

    pub fn push_transform(&mut self, mat: Mat4) {
        let transform = match self.transform_stack.last() {
            Some(transform) => mat * (*transform), // left-multiply new transformation!
            None => mat,
        };
        self.transform_stack.push(transform);
    }

    pub fn pop_transform(&mut self) {
        match self.transform_stack.pop() {
            Some(_) => panic!("tried to pop an empty transform stack"),
            None => {},
        }
    }

    pub fn build_camera(&self) -> Camera {
        Camera::look_at(
            require_optional!(self, camera_position),
            require_optional!(self, camera_up),
            require_optional!(self, camera_look_at),
        )
    }

    pub fn build_render_parameters(&self) -> RenderParamaters {
        RenderParamaters {
            antialias: self.antialias.unwrap_or(1),
        }
    }
}
