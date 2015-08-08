extern crate image;

use std::env;
use std::process;

use image::open;
use display::Display;
use parse_args::ArgParser;
use parse_args::Error as ParseError;

mod display;
mod braille;
mod parse_args;

fn main() {
    match ArgParser::new(env::args().skip(1).collect()) {
        Err(ParseError::TooFewArgs) =>
            complain("Too few arguments!"),
        Err(ParseError::CantParseSize) =>
            complain("Couldn't parse size!"),
        Err(ParseError::BadArgs) =>
            complain("Bad arguments!"),

        Ok(args) => {
            let img = match image::open(args.path()) {
                Ok(image) => image.to_luma(),
                Err(_) => complain("Couldn't open image!")
            };
            let (width, height) = args.size().unwrap_or({
                let (width, height) = img.dimensions();
                (width/10, height/10)
            });

            let display = Display::new(img, width, height);

            println!("{}", display.render());
        }
    }
}

fn complain(error: &str) -> ! {
    let usage = "braillify image [size]\n\
                 \timage: Any image (example: image.png)\n\
                 \tsize: The desired output size. (example: 50x25)\n\
                 \t\tIf a size isn't given, it'll be guessed automatically.";

    println!("{}\n\n{}", error, usage);
    process::exit(1);
}
