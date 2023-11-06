use std::collections::HashMap;

use super::color::Color;

pub type Palette = HashMap<String, Color>;

pub fn into_palette(colors: &[Color]) -> Palette {
    colors
        .iter()
        .enumerate()
        .map(|(index, &color)| (format!("color_{index}"), color))
        .collect()
}
