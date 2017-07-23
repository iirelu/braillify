extern crate image;

use std::env;
use std::process;

use display::Display;
use parse_args::ArgParser;
use parse_args::Error as ParseError;

mod display;
mod braille;
mod parse_args;


const USAGE_TEXT: &'static str = "\
braillify image [size]
    image: Any image (ex: image.png)
    size: The desired output size in characters. (ex: 50x25)
        If a size isn't given, it'll be guessed from the image's dimensions.";

fn main() {
    let args = match ArgParser::new(env::args().skip(1).collect()) {
        Err(ParseError::TooFewArgs) =>
            complain("Too few arguments!"),
        Err(ParseError::CantParseSize) =>
            complain("Couldn't parse size!"),
        Err(ParseError::BadArgs) =>
            complain("Bad arguments!"),
        Ok(args) => args
    };

    let img = match image::open(args.path()) {
        Ok(image) => image.to_luma(),
        Err(_) => complain("Couldn't open image!")
    };

    let (width, height) = args.size().unwrap_or({
        let (width, height) = img.dimensions();
        (width/10, height/20)
    });

    let display = Display::new(img, width, height);

    print!("{}", display.render());
}

fn complain(error: &str) -> ! {
    println!("{}\n\n{}", error, USAGE_TEXT);
    process::exit(1);
}
