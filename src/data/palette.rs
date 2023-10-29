use super::color::Color;

#[derive(Default)]
pub struct Palette {
    color_1: Color,
    color_2: Color,
    color_3: Color,
    color_4: Color,
    color_5: Color,
    color_6: Color,
    color_7: Color,
    color_8: Color,
}

impl IntoIterator for Palette {
    type Item = Color;

    type IntoIter = PaletteIntoInterator;

    fn into_iter(self) -> Self::IntoIter {
        PaletteIntoInterator {
            palette: self,
            index: 0,
        }
    }
}

impl FromIterator<[u8; 3]> for Palette {
    fn from_iter<T: IntoIterator<Item = [u8; 3]>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Palette {
            color_1: iter.next().unwrap_or([0, 0, 0]),
            color_2: iter.next().unwrap_or([0, 0, 0]),
            color_3: iter.next().unwrap_or([0, 0, 0]),
            color_4: iter.next().unwrap_or([0, 0, 0]),
            color_5: iter.next().unwrap_or([0, 0, 0]),
            color_6: iter.next().unwrap_or([0, 0, 0]),
            color_7: iter.next().unwrap_or([0, 0, 0]),
            color_8: iter.next().unwrap_or([0, 0, 0]),
        }
    }
}

pub struct PaletteIntoInterator {
    palette: Palette,
    index: usize,
}

impl Iterator for PaletteIntoInterator {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.palette.color_1,
            1 => self.palette.color_2,
            2 => self.palette.color_3,
            3 => self.palette.color_4,
            4 => self.palette.color_5,
            5 => self.palette.color_6,
            6 => self.palette.color_7,
            7 => self.palette.color_8,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl From<Palette> for Vec<[u8; 3]> {
    fn from(value: Palette) -> Self {
        vec![
            value.color_1,
            value.color_2,
            value.color_3,
            value.color_4,
            value.color_5,
            value.color_6,
            value.color_7,
            value.color_8,
        ]
    }
}
