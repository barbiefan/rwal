use std::{path::Path, process::Command};

use super::{Backend, Color, Palette, WalBackend};

impl Backend for WalBackend {
    fn generate_palette(&self, path: &Path) -> Palette {
        let magick_command = "magick";
        let raw_colors = WalBackend::imagemagick(8, path, magick_command);

        assert!(
            !raw_colors.is_empty(),
            "Imagemagick couldn't generate a suitable palette."
        );

        raw_colors
            .iter()
            .enumerate()
            .map(|(index, &color)| (format!("color_{}", index + 1), color))
            .collect()
    }
}

impl WalBackend {
    fn imagemagick(color_count: i32, img: &Path, magic_command: &str) -> Vec<Color> {
        let path = img.as_os_str().to_str().unwrap();
        let flags = [
            path,
            "-resize",
            "25%",
            "-colors",
            &color_count.to_string(),
            "-unique-colors",
            "txt:-",
        ];

        let output = Command::new(magic_command)
            .args(flags)
            .output()
            .expect("failed to execute imagemagick");
        let colors = output.stdout;
        let colors: Vec<Color> = String::from_utf8(colors)
            .expect("Failed to parse colors")
            .lines()
            .skip(1)
            .map(find_color)
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
    Color::from([r, g, b])
}
