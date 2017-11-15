use std::collections::HashMap;
use std::boxed::Box;
use std::rc::Rc;
use core::*;
use math::*;
use geometry::*;

#[derive(Debug, Default)]
pub struct SceneBuilder {
    camera_position: Option<Point>,
    camera_up: Option<Vec3>,
    camera_look_at: Option<Point>,
    image_dimensions: Option<(u32, u32)>,
    antialias: Option<u32>,
    depth_limit: Option<u32>,
    background_color: Option<Color>,
    materials: HashMap<String, Rc<Material>>,
    // TODO: Should transform be an Rc instead? Feels like this can get expensive.
    transform_stack: Vec<Transform>,
    pub objects: Vec<SceneObject>,
    pub lights: Vec<Light>,
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
    optional_setter!(image_dimensions, (u32, u32));
    optional_setter!(antialias, u32);
    optional_setter!(depth_limit, u32);
    optional_setter!(background_color, Color);

    pub fn register_material(&mut self, name: &str, material: Box<Material>) {
        self.materials.insert(name.to_owned(), Rc::from(material));
    }

    fn get_current_transform(&self) -> Transform {
        match self.transform_stack.last() {
            Some(transform) => *transform,
            None => IDENTITY_TRANSFORM,
        }
    }

    pub fn push_transform(&mut self, mat: Mat4) {
        let new_transform_matrix = mat * self.get_current_transform().m;
        self.transform_stack.push(Transform::new(new_transform_matrix));
    }

    pub fn pop_transform(&mut self) {
        match self.transform_stack.pop() {
            Some(_) => {},
            None => { panic!("tried to pop an empty transform stack"); },
        }
    }

    pub fn add_object(&mut self, partial_object: (Box<Geometry>, &str)) {
        let transform = self.get_current_transform();
        let material_name = partial_object.1.to_owned();
        self.objects.push(SceneObject {
            shape: Shape::new(Rc::from(partial_object.0), transform),
            material: Rc::clone(self.materials.get(&material_name).expect(format!("no material named '{}' defined", material_name).as_str())),
        });
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
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
            image_dimensions: self.image_dimensions.unwrap_or((128u32, 128u32)),
            antialias: self.antialias.unwrap_or(1),
            depth_limit: self.antialias.unwrap_or(3),
            background_color: self.background_color.unwrap_or(BLACK),
        }
    }
}
