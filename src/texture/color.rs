use core::*;

impl Texture for Color {
    fn get_color(&self, _uv: Uv) -> Color {
        self.clone()
    }
}
