#[derive(Default, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Color {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl Color {
    #[must_use]
    pub fn as_dec_rgb(&self) -> String {
        format!("{}, {}, {}", self.r, self.g, self.b)
    }
    #[must_use]
    pub fn as_hex_code(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
