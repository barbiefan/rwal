use std::collections::HashMap;

use super::color::Color;

pub type Palette = HashMap<String, Color>;

#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn into_palette(colors: &[Color]) -> Palette {
    colors
        .iter()
        .enumerate()
        .map(|(index, &color)| (format!("color_{index}"), color))
        .collect()
}
