use super::color::Color;

#[derive(Debug)]
pub struct RenderParamaters {
    pub image_dimensions: (u32, u32),
    pub antialias: u32,
    pub antialias_tolerance: f64,
    pub depth_limit: u32,
    pub background_color: Color,
}
