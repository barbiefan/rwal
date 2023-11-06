use super::{BgFgRestPattern, Color, Pattern};

impl Pattern for BgFgRestPattern {
    /// Puts darkest color in 1st place, then brightest color in 2nd, then puts the rest not
    /// changing its order
    /// current implementation is ass.
    fn shape<'a, 'b>(&'a self, colors: &'b mut [Color]) -> &'b [Color] {
        let length = colors.len();
        if length < 2 {
            panic!("can't apply BgFgRest pattern because there's less than 2 colors")
        };
        let mut temp: Vec<_> = colors
            .iter()
            .enumerate()
            .map(|(index, color)| {
                (
                    i64::try_from(index)
                        .expect(&format!("can't apply BgFgRest on color size {length}")),
                    color,
                )
            })
            .collect();
        let mut temp2 = temp.clone();

        temp.sort_by_key(|c| c.1.brightness());
        let dr = temp.first().unwrap();
        let br = temp.last().unwrap();

        temp2
            .get_mut(usize::try_from(dr.0).expect("can't apply BgFgRest on color size {length}"))
            .expect("can't apply BgFgRest - can't get index of darkest color")
            .0 = -2;
        temp2
            .get_mut(usize::try_from(br.0).expect("can't apply BgFgRest on color size {length}"))
            .expect("can't apply BgFgRest - can't get index of darkest color")
            .0 = -1;
        temp2.sort_by_key(|e| e.0);
        let temp2: Vec<_> = temp2.iter().map(|e| *e.1).collect();

        colors.clone_from_slice(&temp2[..]);
        colors
    }
}
