use super::{Color, Pattern, RandomPattern};

impl Pattern for RandomPattern {
    fn shape<'a, 'b>(&'a self, colors: &'b mut [Color]) -> &'b [Color] {
        colors.sort_by_key(|_| rand::random::<i64>());
        colors
    }
}
