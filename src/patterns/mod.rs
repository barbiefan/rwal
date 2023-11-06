use super::data::color::Color;

pub mod bgfgrest;
pub mod brightness;
pub mod grayscalefirst;
pub mod random;

pub trait Pattern {
    fn shape<'b>(&self, colors: &'b mut [Color]) -> &'b [Color];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Patterns {
    Brightness,
    Random,
    BgFgRest,
    GrayScaleFirst,
}

pub fn get_patterns(patterns: &Vec<Patterns>) -> Vec<Box<dyn Pattern>> {
    let mut out: Vec<Box<dyn Pattern>> = vec![];
    for pattern in patterns {
        out.push(match pattern {
            Patterns::Brightness => Box::new(BrightnessPattern {}),
            Patterns::Random => Box::new(RandomPattern {}),
            Patterns::BgFgRest => Box::new(BgFgRestPattern {}),
            Patterns::GrayScaleFirst => Box::new(GrayScaleFirstPattern {}),
        });
    }
    out
}

#[derive(Debug)]
pub struct BrightnessPattern;
#[derive(Debug)]
pub struct RandomPattern;
#[derive(Debug)]
pub struct BgFgRestPattern;
#[derive(Debug)]
pub struct GrayScaleFirstPattern;
