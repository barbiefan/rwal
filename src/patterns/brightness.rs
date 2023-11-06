use super::{BrightnessPattern, Color, Pattern};

impl Pattern for BrightnessPattern {
    fn shape<'a, 'b>(&'a self, colors: &'b mut [Color]) -> &'b [Color] {
        colors.sort_by_key(|c| c.brightness());
        colors
    }
}
