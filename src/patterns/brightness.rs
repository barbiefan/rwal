use super::{BrightnessPattern, Color, Pattern};

impl Pattern for BrightnessPattern {
    fn shape<'b>(&self, colors: &'b mut [Color]) -> &'b [Color] {
        colors.sort_by_key(Color::brightness);
        colors
    }
}
