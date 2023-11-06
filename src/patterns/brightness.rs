use std::collections::HashSet;

use crate::data::palette::Palette;

use super::{BrightnessPattern, Color, Pattern};

impl Pattern for BrightnessPattern {
    fn shape(&self, colors: &HashSet<Color>) -> Palette {
        let mut v: Vec<_> = colors.iter().collect();
        v.sort_by_key(|c| c.brightness());
        v.iter()
            .enumerate()
            .map(|(index, &&color)| (format!("color_{index}"), color))
            .collect()
    }
}
