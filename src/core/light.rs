use math::*;
use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Point,
    pub color: Color,
}
