use image::{
    imageops::{self, FilterType::Nearest},
    DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage,
};
use rwal::{
    backends::{Backend, Backends, MedianCut, SimpleBackend, WalBackend},
    patterns::{BrightnessPattern, Pattern, Patterns},
    templating::template::process_templates,
};
use std::path::{Path, PathBuf};

use clap::{error::ErrorKind, CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    file_path: PathBuf,

    #[arg(short, long, default_value = "median-cut", value_enum)]
    backend: Backends,

    #[arg(short, long, default_value = "brightness", value_enum)]
    pattern: Patterns,

    #[arg(short, long, default_value = "16", value_parser=validate_colors_number)]
    colors: usize,

    #[arg(short, long, default_value = "false")]
    test: bool,
}

fn main() {
    let templates_dir = Path::new("/home/obey/.config/rwal/templates");
    let cache_dir = Path::new("/home/obey/.cache/rwal/");

    let arguments = Arguments::parse();

    let backend: Box<dyn Backend> = match &arguments.backend {
        Backends::Simple => Box::new(SimpleBackend {}),
        Backends::Wal => Box::new(WalBackend {}),
        Backends::MedianCut => Box::new(MedianCut {}),
    };

    let pattern: Box<dyn Pattern> = match &arguments.pattern {
        Patterns::Brightness => Box::new(BrightnessPattern {}),
    };

    let mut cmd = Arguments::command();
    match imghdr::from_file(&arguments.file_path) {
        Ok(opt) => match opt {
            Some(t) => match t {
                imghdr::Type::Jpeg | imghdr::Type::Png => (),
                _ => {
                    cmd.error(
                        ErrorKind::ValueValidation,
                        format!(
                            "{}: Unsupported image type <{t:?}>",
                            arguments.file_path.to_string_lossy()
                        ),
                    )
                    .exit();
                }
            },
            None => {
                cmd.error(
                    ErrorKind::ValueValidation,
                    format!(
                        "{}: Provided file is not an image.",
                        arguments.file_path.to_string_lossy()
                    ),
                )
                .exit();
            }
        },
        Err(err) => {
            cmd.error(
                ErrorKind::ValueValidation,
                format!("{}: {err}", arguments.file_path.to_string_lossy()),
            )
            .exit();
        }
    };

    let colors = backend.generate_colors(&arguments.file_path, arguments.colors);
    let pal = pattern.shape(&colors);

    if arguments.test {
        let mut orig = image::open(&arguments.file_path).expect("expected valid png or jpeg image");
        let (o_width, o_heigth) = orig.dimensions();
        let width = o_width / 10;

        let mut pimg: RgbaImage = ImageBuffer::new(
            1,
            u32::try_from(arguments.colors).expect("colors number bigger than u32 range"),
        );
        for (index, pix) in pimg.pixels_mut().enumerate() {
            let pp = pal[&format!("color_{index}")];
            *pix = Rgba::from([pp.r, pp.g, pp.b, 255]);
        }
        let mut pimg: DynamicImage = pimg.into();
        pimg = pimg.resize_exact(width, o_heigth, Nearest);

        imageops::overlay(&mut orig, &pimg, 0, -1);
        orig.save(format!(
            "/home/obey/Documents/git/rwal/test/{}",
            arguments.file_path.file_name().unwrap().to_string_lossy()
        ))
        .expect("can't save image");
    } else {
        match process_templates(&pal, templates_dir, cache_dir) {
            Ok(_) => (),
            Err(err) => {
                cmd.error(ErrorKind::Io, format!("{err}")).exit();
            }
        }
    }
}

fn validate_colors_number(s: &str) -> Result<usize, String> {
    clap_num::number_range(s, 0, 256)
}
