use std::collections::HashMap;
use std::boxed::Box;
use std::rc::Rc;
use core::*;
use math::*;
use geometry::*;

#[derive(Debug, Default)]
pub struct SceneBuilder {
    image_dimensions: Option<(u32, u32)>,
    antialias: Option<u32>,
    depth_limit: Option<u32>,
    background_color: Option<Color>,
    textures: HashMap<String, Rc<Texture>>,
    // TODO: Should transform be an Rc instead? Feels like this can get expensive.
    transform_stack: Vec<Transform>,
    camera: Option<Camera>,
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

    optional_setter!(camera, Camera);
    optional_setter!(image_dimensions, (u32, u32));
    optional_setter!(antialias, u32);
    optional_setter!(depth_limit, u32);
    optional_setter!(background_color, Color);

    pub fn register_texture(&mut self, name: &str, texture: Box<Texture>) {
        self.textures.insert(name.to_owned(), Rc::from(texture));
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

    pub fn pop_transforms(&mut self, count: u32) {
        for _ in 0..count {
            match self.transform_stack.pop() {
                Some(_) => {},
                None => { panic!("tried to pop an empty transform stack"); },
            };
        }
    }

    pub fn add_object(&mut self, partial_object: (Box<Geometry>, &str)) {
        let transform = self.get_current_transform();
        let texture_name = partial_object.1.to_owned();
        self.objects.push(SceneObject {
            shape: Shape::new(Rc::from(partial_object.0), transform),
            texture: Rc::clone(self.textures.get(&texture_name).expect(format!("no texture named '{}' defined", texture_name).as_str())),
        });
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn build_camera(&self) -> Camera {
        require_optional!(self, camera)
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
