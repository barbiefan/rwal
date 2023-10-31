use super::color::Color;

#[derive(Default)]
pub struct Palette {
    pub color_1: Color,
    pub color_2: Color,
    pub color_3: Color,
    pub color_4: Color,
    pub color_5: Color,
    pub color_6: Color,
    pub color_7: Color,
    pub color_8: Color,
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
            color_1: iter.next().unwrap_or([0, 0, 0]).into(),
            color_2: iter.next().unwrap_or([0, 0, 0]).into(),
            color_3: iter.next().unwrap_or([0, 0, 0]).into(),
            color_4: iter.next().unwrap_or([0, 0, 0]).into(),
            color_5: iter.next().unwrap_or([0, 0, 0]).into(),
            color_6: iter.next().unwrap_or([0, 0, 0]).into(),
            color_7: iter.next().unwrap_or([0, 0, 0]).into(),
            color_8: iter.next().unwrap_or([0, 0, 0]).into(),
        }
    }
}

impl FromIterator<Color> for Palette {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Palette {
            color_1: iter.next().unwrap_or(Color::default()),
            color_2: iter.next().unwrap_or(Color::default()),
            color_3: iter.next().unwrap_or(Color::default()),
            color_4: iter.next().unwrap_or(Color::default()),
            color_5: iter.next().unwrap_or(Color::default()),
            color_6: iter.next().unwrap_or(Color::default()),
            color_7: iter.next().unwrap_or(Color::default()),
            color_8: iter.next().unwrap_or(Color::default()),
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
            [value.color_1.r, value.color_1.g, value.color_1.b],
            [value.color_2.r, value.color_2.g, value.color_2.b],
            [value.color_3.r, value.color_3.g, value.color_3.b],
            [value.color_4.r, value.color_4.g, value.color_4.b],
            [value.color_5.r, value.color_5.g, value.color_5.b],
            [value.color_6.r, value.color_6.g, value.color_6.b],
            [value.color_7.r, value.color_7.g, value.color_7.b],
            [value.color_8.r, value.color_8.g, value.color_8.b],
        ]
    }
}
