use std::collections::HashSet;
use std::fmt::Display;
use std::path::Path;

use super::data::color::Color;

pub mod median_cut;
pub mod simple;
pub mod wal;

pub trait Backend {
    fn generate_colors(&self, path: &Path, colors: usize) -> HashSet<Color>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Backends {
    Simple,
    Wal,
    MedianCut,
}

impl Display for Backends {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backends::Simple => f.write_str("simple"),
            Backends::Wal => f.write_str("wal"),
            Backends::MedianCut => f.write_str("mediancut"),
        }
    }
}

#[must_use]
pub fn get_backend(back: &Backends) -> Box<dyn Backend> {
    match back {
        Backends::Simple => Box::new(SimpleBackend {}),
        Backends::Wal => Box::new(WalBackend {}),
        Backends::MedianCut => Box::new(MedianCut {}),
    }
}

#[derive(Debug)]
pub struct SimpleBackend;
#[derive(Debug)]
pub struct WalBackend;
#[derive(Debug)]
pub struct MedianCut;
