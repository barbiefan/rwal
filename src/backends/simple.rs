use std::{collections::HashMap, path::Path};

use image::imageops::FilterType;
use image::GenericImageView;

use super::{Backend, Color, Palette, SimpleBackend};

impl Backend for SimpleBackend {
    fn generate_palette(&self, path: &Path) -> Palette {
        let file = image::open(path)
            .unwrap_or_default()
            .resize(128, 128, FilterType::Gaussian);
        let mut pix_map: HashMap<Color, i32> = HashMap::default();
        file.pixels()
            .map(|pixel| Color::from([pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]]))
            .for_each(|data| {
                *pix_map.entry(data).or_insert(0) += 1;
            });
        let mut pix_vec: Vec<_> = pix_map.into_iter().collect();
        pix_vec.sort_by_key(|(_, count)| *count);
        pix_vec.reverse();
        pix_vec[0..=9]
            .iter()
            .map(|(col, _)| col)
            .enumerate()
            .map(|(index, &color)| (format!("color_{}", index + 1), color))
            .collect()
    }
}
