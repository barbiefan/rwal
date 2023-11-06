use std::hash::Hash;

#[derive(Debug, Default, Eq, Hash, PartialEq, Copy, Clone)]
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
    pub fn brightness(&self) -> u8 {
        (0.299 * (f64::from(self.r).powi(2))
            + 0.587 * (f64::from(self.g).powi(2))
            + 0.114 * (f64::from(self.b).powi(2)))
        .powf(0.5) as u8
    }
    #[must_use]
    pub fn as_dec_rgb(&self) -> String {
        format!("{}, {}, {}", self.r, self.g, self.b)
    }
    #[must_use]
    pub fn as_hex_code(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
