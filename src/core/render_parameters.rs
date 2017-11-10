use super::color::Color;

#[derive(Debug)]
pub struct RenderParamaters {
    pub antialias: u32,
    pub depth_limit: u32,
    pub background_color: Color,
}
