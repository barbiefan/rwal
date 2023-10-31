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
    pub fn as_decimal_tuple(&self) -> String {
        format!("({}, {}, {})", self.r, self.g, self.b)
    }
}
