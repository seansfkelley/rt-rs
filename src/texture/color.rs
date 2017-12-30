use core::*;

impl Texture for Color {
    fn get_color(&self, _uv: Option<Uv>) -> Color {
        self.clone()
    }
}
