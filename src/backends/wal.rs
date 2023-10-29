use std::process::Command;

use super::{Backend, Color, Palette};

#[derive(Debug)]
pub struct WalBackend {}

impl Backend for WalBackend {
    fn generate_palette(&self, path: &str) -> Palette {
        let magick_command = "magick";
        let mut raw_colors: Vec<Color> = Vec::new();
        for i in 0..20 {
            raw_colors = WalBackend::imagemagick(16 + i, path, magick_command);
            if raw_colors.len() > 7 {
                break;
            }
        }

        if raw_colors.is_empty() {
            panic!("Imagemagick couldn't generate a suitable palette.")
        }

        raw_colors.into_iter().collect()
    }
}

impl WalBackend {
    fn imagemagick(color_count: i32, img: &str, magic_command: &str) -> Vec<Color> {
        let flags = [
            img,
            "-resize",
            "25%",
            "-colors",
            &color_count.to_string(),
            "-unique-colors",
            "txt:-",
        ];

        let output = Command::new(magic_command)
            .args(&flags)
            .output()
            .expect("failed to execute imagemagick");
        let colors = output.stdout;
        let colors: Vec<Color> = String::from_utf8(colors)
            .expect("Failed to parse colors")
            .lines()
            .skip(1)
            .map(|line| find_color(line))
            .collect();
        colors
    }
}

fn find_color(line: &str) -> Color {
    let start = line.find('#').unwrap();
    from_hex(&line[start..start + 7])
}

fn from_hex(rgb_hex: &str) -> Color {
    let r = u8::from_str_radix(&rgb_hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&rgb_hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&rgb_hex[5..], 16).unwrap();
    [r, g, b]
}
