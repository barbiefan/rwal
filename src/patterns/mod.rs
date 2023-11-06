use std::collections::HashSet;

use crate::data::palette::Palette;

use super::data::color::Color;

pub mod brightness;

pub trait Pattern {
    fn shape(&self, colors: &HashSet<Color>) -> Palette;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Patterns {
    Brightness,
}

#[derive(Debug)]
pub struct BrightnessPattern;
