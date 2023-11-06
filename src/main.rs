use image::{
    imageops::FilterType::Nearest, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage,
};
use rwal::{
    backends::{get_backend, Backends},
    data::palette::into_palette,
    patterns::{get_patterns, Patterns},
    templating::template::process_templates,
};
use std::path::PathBuf;

use clap::{error::ErrorKind, CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    file_path: PathBuf,

    #[arg(short, long, default_value = "median-cut", value_enum)]
    backend: Backends,

    #[arg(short, long, default_value = "brightness", value_delimiter = ',')]
    patterns: Vec<Patterns>,

    #[arg(short, long, default_value = "16", value_parser=validate_colors_number)]
    colors: usize,

    #[arg(short, long, default_value = "false")]
    test: bool,

    #[arg(short, long, default_value = None)]
    templates_dir: Option<String>,
    #[arg(short, long, default_value = None)]
    cache_dir: Option<String>,
}

/// main is a mess rn
fn main() {
    let arguments = Arguments::parse();

    let backend = get_backend(&arguments.backend);
    let patterns = get_patterns(&arguments.patterns);

    let mut cmd = Arguments::command();

    let mut templates_dir: PathBuf;
    let mut cache_dir: PathBuf;

    // idk how to do this gracefuly
    if arguments.templates_dir.is_none() || arguments.cache_dir.is_none() {
        let home_env = match std::env::var("HOME") {
            Err(err) => cmd.error(ErrorKind::Io, format!("{err}")).exit(),
            Ok(p) => p,
        };
        let home_path = PathBuf::from(&home_env);
        templates_dir = home_path.clone();
        cache_dir = home_path.clone();
        templates_dir.push("/.config/rwal/templates");
        cache_dir.push("/.cache/rwal/");
        if let Some(d) = &arguments.templates_dir {
            templates_dir = PathBuf::from(d);
        };
        if let Some(d) = &arguments.cache_dir {
            cache_dir = PathBuf::from(d);
        };
    } else {
        templates_dir = PathBuf::from(arguments.templates_dir.clone().expect("hmm..."));
        cache_dir = PathBuf::from(arguments.cache_dir.clone().expect("hmm..."));
    }

    let templates_dir = templates_dir.as_path();
    let cache_dir = cache_dir.as_path();

    check_image_file(&arguments, &mut cmd);

    let colors = backend.generate_colors(&arguments.file_path, arguments.colors);
    let mut colors = colors.into_iter().collect::<Vec<_>>();
    for pattern in patterns {
        colors = pattern.shape(&mut colors).to_vec();
    }
    let pal = into_palette(&colors);

    if arguments.test {
        let filename = arguments.file_path.file_name().unwrap().to_string_lossy();
        let filename = format!("{}_{}", &arguments.backend, filename);

        let mut pimg: RgbaImage = ImageBuffer::new(
            u32::try_from(arguments.colors).expect("colors number bigger than u32 range"),
            1,
        );
        for (index, pix) in pimg.pixels_mut().enumerate() {
            let pp = pal[&format!("color_{index}")];
            *pix = Rgba::from([pp.r, pp.g, pp.b, 255]);
        }
        let mut pimg: DynamicImage = pimg.into();
        let (w, h) = pimg.dimensions();
        pimg = pimg.resize_exact(w * 300, h * 300, Nearest);

        pimg.save(format!("/home/obey/Documents/git/rwal/test/{filename}",))
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

fn check_image_file(arguments: &Arguments, cmd: &mut clap::Command) {
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
}

fn validate_colors_number(s: &str) -> Result<usize, String> {
    clap_num::number_range(s, 1, 256)
}
