use std::path::Path;

use image::{imageops::FilterType, GenericImageView};

use super::{Backend, Color, MedianCut, Palette};

impl Backend for MedianCut {
    fn generate_palette(&self, path: &Path) -> Palette {
        let pixels: Vec<Color> = image::open(path)
            .unwrap_or_default()
            .resize_exact(1920, 1080, FilterType::Nearest)
            .pixels()
            .map(|pixel| Color::from([pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]]))
            .collect();
        Self::process(pixels)
    }
}

impl MedianCut {
    fn process_buckets(buckets: &mut Vec<Bucket>, to_divide: usize) -> &mut Vec<Bucket> {
        if to_divide == buckets.len() {
            return buckets;
        }
        let buck = find_bucket_to_divide(buckets);
        let (b1, b2) = buck.divide_on_median();
        let buck_index = buckets
            .iter()
            .position(|b| b.pixels == buck.pixels)
            .unwrap();
        buckets.remove(buck_index);
        buckets.push(b1);
        buckets.push(b2);
        return Self::process_buckets(buckets, to_divide);
    }

    fn process(pixels: Vec<Color>) -> Palette {
        let num_colors = 16;
        let bucket = Bucket::new(pixels);
        let mut buckets = vec![bucket];
        let buckets = Self::process_buckets(&mut buckets, num_colors);
        buckets
            .iter()
            .map(Bucket::find_avg_color)
            .enumerate()
            .map(|(index, color)| (format!("color_{}", index), color))
            .collect()
    }
}

#[derive(Debug)]
enum Channel {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Bucket {
    pixels: Vec<Color>,
    range_r: [u8; 2],
    range_g: [u8; 2],
    range_b: [u8; 2],
    biggest_range: Option<Channel>,
}

impl Bucket {
    fn new(pixels: Vec<Color>) -> Self {
        let mut b = Self {
            pixels,
            range_r: [0, 0],
            range_g: [0, 0],
            range_b: [0, 0],
            biggest_range: None,
        };
        b.find_bucket_ranges();
        b.sort();
        b
    }

    fn find_avg_color(&self) -> Color {
        let count = self.pixels.len();
        self.pixels
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

    fn find_bucket_ranges(&mut self) {
        for pix in &self.pixels[..] {
            if self.range_r[0] > pix.r {
                self.range_r[0] = pix.r;
            };
            if self.range_r[1] < pix.r {
                self.range_r[1] = pix.r;
            };
            if self.range_g[0] > pix.g {
                self.range_g[0] = pix.g;
            };
            if self.range_g[1] < pix.g {
                self.range_g[1] = pix.g;
            };
            if self.range_b[0] > pix.b {
                self.range_b[0] = pix.b;
            };
            if self.range_b[1] < pix.b {
                self.range_b[1] = pix.b;
            };
        }
        let rr = range(self.range_r);
        let rg = range(self.range_g);
        let rb = range(self.range_b);
        let biggest = match [rr, rg, rb].iter().max() {
            Some(&x) if x == rr => Channel::Red,
            Some(&x) if x == rg => Channel::Green,
            Some(&x) if x == rb => Channel::Blue,
            Some(&x) => {
                panic!("can't find biggest channel. r({rr}) g({rg}) b({rb}), comapred against {x}")
            }
            None => panic!("can't find biggest channel. r({rr}) g({rg}) b({rb}), but max is None"),
        };
        self.biggest_range = Some(biggest);
    }

    fn divide_on_median(&self) -> (Bucket, Bucket) {
        let median = self.pixels.len() / 2;
        (
            Bucket::new(self.pixels[0..median].into()),
            Bucket::new(self.pixels[median..].into()),
        )
    }

    fn sort(&mut self) {
        if let Some(biggest_range) = &self.biggest_range {
            match biggest_range {
                Channel::Red => self.pixels.sort_by(|i1, i2| i1.r.cmp(&i2.r)),
                Channel::Green => self.pixels.sort_by(|i1, i2| i1.g.cmp(&i2.g)),
                Channel::Blue => self.pixels.sort_by(|i1, i2| i1.b.cmp(&i2.b)),
            }
        }
    }
}

fn find_bucket_to_divide(buckets: &Vec<Bucket>) -> &Bucket {
    if buckets.len() == 1 {
        return buckets.first().unwrap();
    }
    let mut largest_range = 0;
    let mut biggest_bucket: &Bucket = buckets.first().expect("empty bucket list!");
    for bucket in buckets {
        let rr = range(bucket.range_r);
        let rg = range(bucket.range_g);
        let rb = range(bucket.range_b);
        if largest_range < rr {
            largest_range = rr;
            biggest_bucket = bucket;
        }
        if largest_range < rg {
            largest_range = rg;
            biggest_bucket = bucket;
        }
        if largest_range < rb {
            largest_range = rb;
            biggest_bucket = bucket;
        }
    }
    biggest_bucket
}

#[inline]
fn range(r: [u8; 2]) -> u8 {
    r[1] - r[0]
}
