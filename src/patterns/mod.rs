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

pub fn get_pattern(patt: &Patterns) -> Box<dyn Pattern> {
    match patt {
        Patterns::Brightness => Box::new(BrightnessPattern {}),
    }
}

#[derive(Debug)]
pub struct BrightnessPattern;
