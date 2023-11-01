use std::fmt::Display;
use std::io::Error;
use std::path::Path;

use crate::data::palette::Palette;

pub struct IoError {
    pub error: Error,
    pub path: String,
}

impl Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.error)
    }
}

/// # Errors
///
/// Will Return `std::io::Error` if something with dirs or files goes wrong
pub fn process_templates(
    palette: &Palette,
    templates_dir: &Path,
    cache_dir: &Path,
) -> Result<(), IoError> {
    std::fs::create_dir_all(templates_dir).map_err(|err| IoError {
        error: err,
        path: templates_dir.to_string_lossy().into_owned(),
    })?;
    std::fs::create_dir_all(cache_dir).map_err(|err| IoError {
        error: err,
        path: cache_dir.to_string_lossy().into_owned(),
    })?;
    let templates_iter = std::fs::read_dir(templates_dir).map_err(|err| IoError {
        error: err,
        path: templates_dir.to_string_lossy().into_owned(),
    })?;

    for entry in templates_iter {
        match entry {
            Err(err) => {
                println!("{err}");
                continue;
            }
            Ok(entry) => {
                if let Ok(ftype) = entry.file_type() {
                    if ftype.is_dir() {
                        process_templates(palette, &entry.path(), cache_dir)?;
                        continue;
                    } else if ftype.is_file() {
                        let mut template =
                            std::fs::read_to_string(&entry.path()).map_err(|err| IoError {
                                error: err,
                                path: entry.path().to_string_lossy().into_owned(),
                            })?;
                        template.colorize(palette);
                        if let Some(template_name) = &entry.path().file_name() {
                            std::fs::write(cache_dir.join(template_name), template).map_err(
                                |err| IoError {
                                    error: err,
                                    path: cache_dir
                                        .join(template_name)
                                        .to_string_lossy()
                                        .into_owned(),
                                },
                            )?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

trait Colorize {
    fn colorize(&mut self, palette: &Palette);
}

impl Colorize for String {
    fn colorize(&mut self, palette: &Palette) {
        for (name, color) in palette {
            *self = self.replace(&format!("{{{name} dec_tuple}}"), &color.as_dec_rgb());
            *self = self.replace(&format!("{{{name} hex_code}}"), &color.as_hex_code());
        }
    }
}
