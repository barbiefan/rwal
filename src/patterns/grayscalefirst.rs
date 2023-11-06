use super::{Color, GrayScaleFirstPattern, Pattern};

impl Pattern for GrayScaleFirstPattern {
    fn shape<'b>(&self, colors: &'b mut [Color]) -> &'b [Color] {
        colors.sort_by(|c1, c2| c1.gray_scale_coeff().total_cmp(&c2.gray_scale_coeff()));
        colors
    }
}
