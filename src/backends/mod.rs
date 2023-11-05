use std::path::Path;

use super::data::color::Color;
use super::data::palette::Palette;

pub mod median_cut;
pub mod simple;
pub mod wal;

pub trait Backend {
    fn generate_palette(&self, path: &Path, colors: usize) -> Palette;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Backends {
    Simple,
    Wal,
    MedianCut,
}

#[derive(Debug)]
pub struct SimpleBackend;
#[derive(Debug)]
pub struct WalBackend;
#[derive(Debug)]
pub struct MedianCut;
