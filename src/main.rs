mod color_codes;
mod file_config;
use color_codes::Colors;
extern crate colored;
use colored::*;
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();
    let config = file_config::TifConfig::new(buffer);
    let mut line_counter: u8 = 0;
    for i in config.pixels {
        let pixel_as_str = match i.color {
            Colors::BLACK => "█".black(),
            Colors::BLUE => "█".bright_blue(),
            Colors::CYAN => "█".cyan(),
            Colors::GREEN => "█".green(),
            Colors::PURPLE => "█".purple(),
            Colors::RED => "█".red(),
            Colors::UNEXPECTED => "█".black(),
            Colors::WHITE => "█".white(),
            Colors::YELLOW => "█".yellow(),
        };

        for _j in 0..i.number_of_pixels {
            line_counter += 1;
            print!("{}", &pixel_as_str);
            if line_counter == config.pixels_per_line {
                line_counter = 0;
                return
            }
        }
    }
}
