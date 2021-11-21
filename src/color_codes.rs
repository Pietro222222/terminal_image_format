use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Colors {
    BLUE,
    RED,
    GREEN,
    YELLOW,
    BLACK,
    PURPLE,
    CYAN,
    WHITE,
    UNEXPECTED,
}

pub fn convert_byte_into_color(byte: u8) -> Colors {
    match byte {
        90 => Colors::BLUE,
        91 => Colors::BLACK,
        92 => Colors::RED,
        93 => Colors::GREEN,
        94 => Colors::PURPLE,
        95 => Colors::WHITE,
        96 => Colors::YELLOW,
        97 => Colors::CYAN,
        _ => Colors::UNEXPECTED,
    }
}

#[derive(Clone)]
pub struct Pixel {
    pub color: Colors,
    pub number_of_pixels: u8,
}
