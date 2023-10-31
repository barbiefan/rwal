use std::path::Path;

use crate::data::palette::Palette;

pub fn process_templates(
    palette: &Palette,
    templates_dir: &Path,
    cache_dir: &Path,
) -> std::io::Result<()> {
    std::fs::create_dir_all(templates_dir)?;
    std::fs::create_dir_all(cache_dir)?;
    let templates_iter = std::fs::read_dir(templates_dir)?;

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
                        let template = std::fs::read_to_string(&entry.path())?.colorize(palette);
                        if let Some(template_name) = &entry.path().file_name() {
                            std::fs::write(cache_dir.join(template_name), template)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

trait Colorize {
    fn colorize(&self, palette: &Palette) -> String;
}

impl Colorize for String {
    fn colorize(&self, palette: &Palette) -> Self {
        self.replace("{color_1 dec_tuple}", &palette.color_1.as_decimal_tuple())
            .replace("{color_2 dec_tuple}", &palette.color_2.as_decimal_tuple())
            .replace("{color_3 dec_tuple}", &palette.color_3.as_decimal_tuple())
            .replace("{color_4 dec_tuple}", &palette.color_4.as_decimal_tuple())
            .replace("{color_5 dec_tuple}", &palette.color_5.as_decimal_tuple())
            .replace("{color_6 dec_tuple}", &palette.color_6.as_decimal_tuple())
            .replace("{color_7 dec_tuple}", &palette.color_7.as_decimal_tuple())
            .replace("{color_8 dec_tuple}", &palette.color_8.as_decimal_tuple())
    }
}
