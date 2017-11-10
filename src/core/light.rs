use math::*;
use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Point,
    pub color: Color,
}

impl Light {
    pub fn new(position: Point, color: Color) -> Light {
        Light { position, color }
    }
}
