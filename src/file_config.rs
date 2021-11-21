

use super::color_codes;
pub struct TifConfig {
    pub pixels_per_line: u8,
    pub pixels: Vec<color_codes::Pixel>,
}

impl TifConfig {
    pub fn new(file_content: Vec<u8>) -> TifConfig {
        let header = &file_content[0..5];
        let mut config = TifConfig {
            pixels: vec![],
            pixels_per_line: 0,
        };
        if header != &[46, 84, 73, 70, 32] {
            panic!("Format File ERROR. is this a tif file?");
        } else {
            let ppl = &file_content[5];
            config.pixels_per_line = *ppl;
            let mut pixel: color_codes::Pixel = color_codes::Pixel {
                color: color_codes::Colors::UNEXPECTED,
                number_of_pixels: 0,
            };
            for i in file_content[6..].into_iter() {
                let color = color_codes::convert_byte_into_color(i.clone());
                if color == color_codes::Colors::UNEXPECTED {
                    //might be the number of pixels
                    if pixel.color == color_codes::Colors::UNEXPECTED {
                        panic!("ERROR: invalid TIF format");
                        //it was supposed to be a color, but it wasnt
                    } else {
                        pixel.number_of_pixels = *i;
                        config.pixels.push(pixel.clone());
                        pixel.color = color_codes::Colors::UNEXPECTED;
                        pixel.number_of_pixels = 1;
                    }
                } else {
                    if pixel.color != color_codes::Colors::UNEXPECTED {
                        pixel.number_of_pixels = 1;
                        config.pixels.push(pixel.clone());
                        pixel.color = color;
                    } else {
                        pixel.color = color;
                    }
                }
            }
        }
        config
    }
}
