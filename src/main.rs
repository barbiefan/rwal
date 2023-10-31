use imghdr;
use rwal::{
    backends::{simple::SimpleBackend, wal::WalBackend, Backend, Backends},
    templating::template::process_templates,
};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fancify")]
struct Arguments {
    #[structopt(parse(from_os_str))]
    file_path: PathBuf,

    #[structopt(short, long, default_value = "Wal")]
    backend: Backends,
}

fn main() {
    let templates_dir = Path::new("/home/obey/.config/rwal/");
    let cache_dir = Path::new("/home/obey/.cache/rwal/");

    let arguments = Arguments::from_args();
    let backend: Box<dyn Backend> = match &arguments.backend {
        Backends::Simple => Box::new(SimpleBackend {}),
        Backends::Wal => Box::new(WalBackend {}),
    };

    match verify_image_path(&arguments.file_path) {
        Err(err) => {
            println!("{err}");
            return;
        }
        _ => (),
    }

    let pal = backend.generate_palette(&arguments.file_path);
    match process_templates(&pal, templates_dir, cache_dir) {
        Ok(_) => (),
        Err(err) => println!("{err}"),
    }
}

fn verify_image_path(path: &PathBuf) -> Result<(), String> {
    match imghdr::from_file(&path) {
        Ok(opt) => match opt {
            Some(t) => match t {
                imghdr::Type::Jpeg | imghdr::Type::Png => Ok(()),
                _ => Err(format!("error: Unsupported image type {:?}", t)),
            },
            None => Err("error: Provided file is not an image".into()),
        },
        Err(err) => Err(format!("error: {}", err)),
    }
}
