use rwal::{
    backends::{Backend, Backends, MedianCut, SimpleBackend, WalBackend},
    data::palette::Palette,
    templating::template::process_templates,
};
use std::path::{Path, PathBuf};

use clap::{error::ErrorKind, CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    file_path: PathBuf,

    #[arg(short, long, default_value = "wal", value_enum)]
    backend: Backends,
}

fn main() {
    let templates_dir = Path::new("/home/obey/.config/rwal/");
    let cache_dir = Path::new("/home/obey/.cache/rwal/");

    let arguments = Arguments::parse();

    let backend: Box<dyn Backend> = match &arguments.backend {
        Backends::Simple => Box::new(SimpleBackend {}),
        Backends::Wal => Box::new(WalBackend {}),
        Backends::MedianCut => Box::new(MedianCut {}),
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

    let pal: Palette = backend.generate_palette(&arguments.file_path);
    match process_templates(&pal, templates_dir, cache_dir) {
        Ok(_) => (),
        Err(err) => {
            cmd.error(ErrorKind::Io, format!("{err}")).exit();
        }
    }
}
