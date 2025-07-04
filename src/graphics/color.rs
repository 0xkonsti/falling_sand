use crate::utils::Flattenable;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Self = Self::hex(0x000000FF);
    pub const BLUE: Self = Self::hex(0x0000FFFF);
    pub const DEEP_DARK_BLUE: Self = Self::hex(0x10101AFF);
    pub const GREEN: Self = Self::hex(0x00FF00FF);
    pub const RED: Self = Self::hex(0xFF0000FF);
    pub const WHITE: Self = Self::hex(0xFFFFFFFF);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: Self::u8_to_f32(r), g: Self::u8_to_f32(g), b: Self::u8_to_f32(b), a: Self::u8_to_f32(a) }
    }

    pub const fn rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::rgba_u8(r, g, b, 255)
    }

    /// Creates a Color from a 32-bit RGBA Hex value.
    pub const fn hex(rgba: u32) -> Self {
        let r = (rgba >> 24) as u8;
        let g = (rgba >> 16) as u8;
        let b = (rgba >> 8) as u8;
        let a = rgba as u8;
        Self::rgba_u8(r, g, b, a)
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    // ----------------< Private >----------------
    const fn u8_to_f32(value: u8) -> f32 {
        value as f32 / 255.0
    }
}

impl Flattenable<f32> for Color {
    fn flatten(self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }
}
