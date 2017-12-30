use std::fmt::Debug;
use super::color::Color;
use super::uv::Uv;

pub trait Texture: Sync + Send + Debug {
    fn get_color(&self, uv: Option<Uv>) -> Color;
}
