use std::{env, process::exit, io::stdout};
use libtif::{image::TifImage, pixel::PixelColor};
use termion::color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() < 1 {
        println!("Please, provide a file!");
        exit(1);
    }
    let file = TifImage::parse_from_file(args.get(0).unwrap().to_owned())?;
    for (height, pixels) in file.pixels.iter().enumerate() {
        for (width, pixel) in pixels.iter().enumerate() {
            use PixelColor::*;

            let color = match pixel {
                Black   => {
                    format!("{}", color::Bg(color::Black))
                },
                Red     => {
                    format!("{}", color::Bg(color::Red))
                },
                Green   => {
                    format!("{}", color::Bg(color::Green))
                },
                Yellow  => {
                    format!("{}", color::Bg(color::Yellow))
                },
                Blue    => {
                    format!("{}", color::Bg(color::Blue))
                },
                Magenta => {
                    format!("{}", color::Bg(color::Magenta))
                },
                Cyan    => {
                    format!("{}", color::Bg(color::Cyan))
                },
                White   => {
                    format!("{}", color::Bg(color::White))
                },
            };
            print!("{}{} {}", termion::cursor::Goto(width as u16 + 1, height as u16 + 1), color, color::Bg(color::Reset));
        }
    }

    Ok(())
}
