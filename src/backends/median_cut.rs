use std::{collections::HashSet, path::Path};

use image::GenericImageView;

use super::{Backend, Color, MedianCut, Palette};

/// Slightly modified Median-cut algorithm
impl Backend for MedianCut {
    fn generate_palette(&self, path: &Path, colors: usize) -> Palette {
        let image = image::open(path).expect("expected valid png or jpeg image");
        let (width, heigth) = image.dimensions();
        let pixels: Vec<_> = image
            // resizing here serves two purposes:
            // 1 - reducing size (duh) so that algorithm has to do less work.
            // 2 - blurring an image to reduce color noise so that random noisy dots don't evlove
            //   into their own bucket.
            .resize_exact(width / 2, heigth / 2, image::imageops::FilterType::Triangle)
            .pixels()
            .map(|pixel| Color::from([pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]]))
            .collect();
        Self::process(&pixels, colors)
    }
}

impl MedianCut {
    fn process_buckets(pixels: &[Color], colors: usize) -> HashSet<Color> {
        const SQ255: f64 = (255 * 255) as f64;
        #[allow(clippy::cast_precision_loss)]
        let initial_pix_length = pixels.len() as f64;
        let mut hashcolors: HashSet<Color> = HashSet::with_capacity(colors);
        let r = find_bucket_ranges(pixels);

        let mut entries: Vec<(Vec<Color>, (Channel, f64), Color)> =
            vec![(pixels.to_owned(), (r.0 .0, 1.0), find_avg_color(pixels))];

        while hashcolors.len() < colors {
            let (index, to_div) = entries
                .iter()
                .enumerate()
                .max_by(|a, b| a.1 .1 .1.total_cmp(&b.1 .1 .1))
                .unwrap();

            // cloning is ok here i guess, since pre-resizeing an image takes almost all of the
            // time. checked with flamegraph
            let (new_bucket_1, new_bucket_2) = divide_on_median(&mut to_div.0.clone(), to_div.1 .0);
            #[allow(clippy::cast_precision_loss)]
            let length_1 = new_bucket_1.len() as f64;
            #[allow(clippy::cast_precision_loss)]
            let length_2 = new_bucket_2.len() as f64;

            let ((channel_1, range_1, range_arr_1), sigma_1) = find_bucket_ranges(&new_bucket_1);
            let ((channel_2, range_2, range_arr_2), sigma_2) = find_bucket_ranges(&new_bucket_2);
            let (range_arr_1, range_1, sigma_1, range_arr_2, range_2, sigma_2) = (
                [f64::from(range_arr_1[0]), f64::from(range_arr_1[1])],
                f64::from(range_1),
                f64::from(sigma_1),
                [f64::from(range_arr_2[0]), f64::from(range_arr_2[1])],
                f64::from(range_2),
                f64::from(sigma_2),
            );

            // tldr for distance:
            // Suppose Red is widest channel.
            // If avg of Red in this bucket is 150, and
            // middle of range(Red) is 85
            // then we can assume that this bucket contains a lot of colors with red channel near
            // 150 and small red channel values are a fluke.
            // otherwise the distance would be small (like avg is 140 and midrange is 135), and
            // we would want to divide this bucket.
            let color_1 = find_avg_color(&new_bucket_1);
            let color_2 = find_avg_color(&new_bucket_2);
            // distance 1
            let dom_color_1 = match channel_1 {
                Channel::Red => f64::from(color_1.r),
                Channel::Green => f64::from(color_1.g),
                Channel::Blue => f64::from(color_1.b),
            };
            let range_mid_1 = (range_arr_1[0] + range_arr_1[1]) / 2.0;
            let distance_1 = range_mid_1 - dom_color_1;
            let distance_1 = (range_mid_1 - distance_1.abs()) / range_mid_1;
            // distance 2
            let dom_color_2 = match channel_2 {
                Channel::Red => f64::from(color_2.r),
                Channel::Green => f64::from(color_2.g),
                Channel::Blue => f64::from(color_2.b),
            };
            let range_mid_2 = (range_arr_2[0] + range_arr_2[1]) / 2.0;
            let distance_2 = range_mid_2 - dom_color_2;
            let distance_2 = (range_mid_2 - distance_2.abs()) / range_mid_2;

            let coeff1 = (length_1 / initial_pix_length)
                * ((range_1 * range_1) / SQ255)
                * ((sigma_1 * sigma_1) / SQ255)
                * (distance_1);
            let coeff2 = (length_2 / initial_pix_length)
                * ((range_2 * range_2) / SQ255)
                * ((sigma_2 * sigma_2) / SQ255)
                * (distance_2);

            let entry1 = (new_bucket_1, (channel_1, coeff1), color_1);
            let entry2 = (new_bucket_2, (channel_2, coeff2), color_2);
            hashcolors.remove(&to_div.2);
            hashcolors.insert(color_1);
            hashcolors.insert(color_2);
            entries.remove(index);
            entries.push(entry1);
            entries.push(entry2);
        }
        hashcolors
    }

    fn process(pixels: &[Color], colors: usize) -> Palette {
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

fn find_bucket_ranges(pixels: &[Color]) -> ((Channel, u8, [u8; 2]), u8) {
    // simd? I'm not skilled enough
    assert!(
        !pixels.is_empty(),
        "find_bucket_ranges received an empty slice"
    );
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
    let mut ranges = [
        (Channel::Red, rr, range_r),
        (Channel::Green, rg, range_g),
        (Channel::Blue, rb, range_b),
    ];
    ranges.sort_by_key(|r| 255 - r.1);
    let rmax = ranges[0];
    let sigma = (u32::from(ranges[0].1) + u32::from(ranges[1].1) + u32::from(ranges[2].1)) / 3;
    (
        rmax,
        u8::try_from(sigma).unwrap_or_else(|_| panic!("can't cast sigma {sigma:?} to u8")),
    )
}

fn divide_on_median(pixels: &mut [Color], channel: Channel) -> (Vec<Color>, Vec<Color>) {
    let median = pixels.len() / 2;
    let (b1, _, b2) = match channel {
        Channel::Red => pixels.select_nth_unstable_by_key(median, |i| i.r),
        Channel::Green => pixels.select_nth_unstable_by_key(median, |i| i.g),
        Channel::Blue => pixels.select_nth_unstable_by_key(median, |i| i.b),
    };
    (b1.into(), b2.into())
}

#[inline]
fn range(r: [u8; 2]) -> u8 {
    r[1] - r[0]
}
