use vector::Vec3;
use super::scene_object::SceneObject;

pub type Uv = (f64, f64);

pub struct Intersection {
    pub distance: f64,
    pub location: Vec3,
    pub normal: Vec3,
    pub uv: Uv,
}

pub struct Hit<'a> {
    pub enter: Option<Intersection>,
    pub exit: Intersection,
    pub object: &'a (SceneObject + 'a),
    pub debug: bool,
}

impl<'a> Hit<'a> {
    pub fn debug(self, debug: bool) -> Hit<'a> {
        Hit {
            enter: self.enter,
            exit: self.exit,
            object: self.object,
            debug,
        }
    }
}
