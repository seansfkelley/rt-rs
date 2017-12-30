use core::*;

#[derive(Debug, Clone)]
pub struct CheckerboardTexture {
    pub checks_u: u32,
    pub checks_v: u32,
    pub color_one: Color,
    pub color_two: Color,
}

impl Texture for CheckerboardTexture {
    fn get_color(&self, uv: Option<Uv>) -> Color {
        match uv {
            Some(Uv(u, v)) => {
                if (u * self.checks_u as f64) as u32 % 2 == (v * self.checks_v as f64) as u32 % 2 {
                    self.color_one
                } else {
                    self.color_two
                }
            }
            None => { Color::BLACK }
        }
    }
}
