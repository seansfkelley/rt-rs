use core::*;

#[derive(Debug, Clone)]
pub struct DebugTexture;

impl Texture for DebugTexture {
    fn get_color(&self, uv: Uv) -> Color {
        let Uv(u, v) = uv;
        Color::new(u, 0f64, v)
    }
}
