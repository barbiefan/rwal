use image;
use rwal::backends::{simple::SimpleBackend, wal::WalBackend, Backend, Backends};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fancify")]
struct Arguments {
    file_path: String,

    #[structopt(short, long, default_value = "Wal")]
    backend: Backends,
}

fn main() {
    let arguments = Arguments::from_args();
    let backend: Box<dyn Backend> = match &arguments.backend {
        Backends::Simple => Box::new(SimpleBackend {}),
        Backends::Wal => Box::new(WalBackend {}),
    };
    let mut sl = backend.generate_palette(&arguments.file_path).into_iter();
    let mut imgbuf = image::ImageBuffer::new(5, 2);
    for pixel in imgbuf.pixels_mut() {
        *pixel = image::Rgb(sl.next().unwrap_or([0, 0, 0]));
    }
    imgbuf.save("/home/obey/palette.png").unwrap();
}
