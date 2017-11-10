use core::Camera;
use math::*;

#[derive(Debug)]
pub struct SceneBuilder {
    camera_position: Option<Point>,
    camera_up: Option<Vec3>,
    look_at: Option<Point>,
}

macro_rules! optional_setter {
    ($name:ident, $type:ty) => {
        pub fn $name(&mut self, input: $type) {
            self.$name = Some(input);
        }
    };
}

macro_rules! optional_getter {
    ($self_:ident, $name:ident) => {
        if $self_.$name.is_some() {
            $self_.$name.unwrap()
        } else {
            panic!("scene file didn't set $name")
        }
    }
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            camera_position: None,
            camera_up: None,
            look_at: None,
        }
    }

    optional_setter!(camera_position, Point);
    optional_setter!(camera_up, Vec3);
    optional_setter!(look_at, Point);

    pub fn build_camera(&self) -> Camera {
        Camera::look_at(
            optional_getter!(self, camera_position),
            optional_getter!(self, camera_up),
            optional_getter!(self, look_at),
        )
    }
}
