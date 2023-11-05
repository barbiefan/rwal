use std::{collections::HashSet, path::Path};

use image::GenericImageView;

use super::{Backend, Color, MedianCut, Palette};

/// Slightly modified Median-cut algorithm
impl Backend for MedianCut {
    fn generate_palette(&self, path: &Path, colors: usize) -> Palette {
        let image = image::open(path).unwrap_or_default();
        let (width, heigth) = image.dimensions();
        let pixels = image
            .resize_exact(
                width / 10,
                heigth / 10,
                image::imageops::FilterType::Triangle,
            )
            .pixels()
            .map(|pixel| Color::from([pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]]))
            .collect();
        Self::process(pixels, colors)
    }
}

impl MedianCut {
    fn process_buckets<'a>(pixels: Vec<Color>, colors: usize) -> HashSet<Color> {
        let mut hashcolors: HashSet<Color> = HashSet::new();
        let mut entries: Vec<(Vec<Color>, usize, Color)> = vec![(
            pixels.clone(),
            pixels.len() * (find_bucket_ranges(&pixels).1 as usize + 1),
            find_avg_color(&pixels),
        )];
        while hashcolors.len() < colors {
            let (index, to_div) = entries.iter().enumerate().max_by_key(|(_, e)| e.1).unwrap();

            // cloning is ok here i guess, since pre-resizeing an image takes almost all of the
            // time. checked with flamegraph
            let (b1, b2) = divide_on_median(&mut to_div.0.clone());
            let l1 = b1.len();
            let l2 = b2.len();
            let r1 = find_bucket_ranges(&b1).1 as usize + 1;
            let r2 = find_bucket_ranges(&b2).1 as usize + 1;
            let c1 = find_avg_color(&b1);
            let c2 = find_avg_color(&b2);
            let entry1 = (b1, l1 * r1, c1);
            let entry2 = (b2, l2 * r2, c2);
            hashcolors.remove(&to_div.2);
            hashcolors.insert(c1);
            hashcolors.insert(c2);
            entries.remove(index);
            entries.push(entry1);
            entries.push(entry2);
        }
        return hashcolors;
    }

    fn process(pixels: Vec<Color>, colors: usize) -> Palette {
        let new_buckets = Self::process_buckets(pixels, colors);
        let mut colors: Vec<_> = new_buckets.iter().collect();
        colors.sort_by_key(|c| c.brightness());
        colors
            .iter()
            .enumerate()
            .map(|(index, &&color)| (format!("color_{}", index), color))
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum Channel {
    Red,
    Green,
    Blue,
}

fn find_avg_color(pixels: &[Color]) -> Color {
    let count = pixels.len();
    pixels
        .iter()
        .fold([0usize, 0usize, 0usize], |acc, item| {
            [
                acc[0] + (item.r as usize),
                acc[1] + (item.g as usize),
                acc[2] + (item.b as usize),
            ]
        })
        .map(|summed| u8::try_from(summed / count).expect("can't calculate average color"))
        .into()
}

fn find_bucket_ranges(pixels: &[Color]) -> (Channel, u32) {
    let [range_r, range_g, range_b] = pixels.iter().fold(
        [[u8::MAX, u8::MIN], [u8::MAX, u8::MIN], [u8::MAX, u8::MIN]],
        |mut prev, curr| {
            prev[0][0] = prev[0][0].min(curr.r);
            prev[0][1] = prev[0][1].max(curr.r);
            prev[1][0] = prev[1][0].min(curr.g);
            prev[1][1] = prev[1][1].max(curr.g);
            prev[2][0] = prev[2][0].min(curr.b);
            prev[2][1] = prev[2][1].max(curr.b);
            prev
        },
    );
    let rr = range(range_r);
    let rg = range(range_g);
    let rb = range(range_b);
    match [rr, rg, rb].iter().max() {
        Some(&x) if x == rr => (Channel::Red, x),
        Some(&x) if x == rg => (Channel::Green, x),
        Some(&x) if x == rb => (Channel::Blue, x),
        Some(&x) => {
            panic!("can't find biggest channel. r({rr}) g({rg}) b({rb}), comapred against {x}")
        }
        None => panic!("can't find biggest channel. r({rr}) g({rg}) b({rb}), but max is None"),
    }
}

fn divide_on_median(pixels: &mut [Color]) -> (Vec<Color>, Vec<Color>) {
    let biggest_range = find_bucket_ranges(pixels).0;
    let median = pixels.len() / 2;
    let (b1, _, b2) = match biggest_range {
        Channel::Red => pixels.select_nth_unstable_by_key(median, |i| i.r),
        Channel::Green => pixels.select_nth_unstable_by_key(median, |i| i.g),
        Channel::Blue => pixels.select_nth_unstable_by_key(median, |i| i.b),
    };
    (b1.into(), b2.into())
}

#[inline]
fn range(r: [u8; 2]) -> u32 {
    (r[1] - r[0]) as u32
}
