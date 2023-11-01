use std::path::Path;
use std::str::FromStr;

use super::data::color::Color;
use super::data::palette::Palette;

pub mod simple;
pub mod wal;

pub trait Backend {
    fn generate_palette(&self, path: &Path) -> Palette;
}

#[derive(Debug)]
pub enum Backends {
    Simple,
    Wal,
}

impl FromStr for Backends {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "simple" | "Simple" => Ok(Self::Simple),
            "wal" | "Wal" => Ok(Self::Wal),
            _ => Err(format!(
                "Unknown backend \"{s}\". Valid backends: {:?}",
                [Backends::Simple, Backends::Wal,]
            )),
        }
    }
}
