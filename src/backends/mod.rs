use std::path::Path;

use super::data::color::Color;
use super::data::palette::Palette;

pub mod simple;
pub mod wal;

pub trait Backend {
    fn generate_palette(&self, path: &Path) -> Palette;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Backends {
    Simple,
    Wal,
}

#[derive(Debug)]
pub struct SimpleBackend;
#[derive(Debug)]
pub struct WalBackend;
