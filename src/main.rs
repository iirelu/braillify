extern crate image;
extern crate clap;

use std::process;

use display::Display;
use clap::{App, Arg};

mod display;
mod braille;

fn main() {
    let matches = App::new("braillify")
        .about("Converts images into braille text")
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .takes_value(true)
            .help("Manually specify the width of the generated braille")
            .long_help(
"Specifies the width of the rendered braille image. If --height is not also
specified, it is guessed from aspect ratio."))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .takes_value(true)
            .help("Manually specify the height of the generated braille")
            .long_help(
"Specifies the height of the rendered braille image. If --width is not also
specified, it is guessed from aspect ratio."))
        .arg(Arg::with_name("file")
            .required(true)
            .help("Image file to convert"))
        .get_matches();

    let img = match image::open(matches.value_of("file").unwrap()) {
        Ok(image) => image.to_luma(),
        Err(_) => {
            println!("Couldn't open image!");
            process::exit(1);
        }
    };

    let (naive_width, naive_height) = img.dimensions();
    let desired_width: Option<u32> = matches.value_of("width")
        .map(|x| x.parse().unwrap());
    let desired_height: Option<u32> = matches.value_of("height")
        .map(|x| x.parse().unwrap());

    let (width, height) = match (desired_width, desired_height) {
        (None, None) => (naive_width / 10, naive_height / 20),
        (Some(w), None) => (w, ((naive_height * w) / naive_width) / 2),
        (None, Some(h)) => (((naive_width * h) / naive_height) * 2, h),
        (Some(w), Some(h)) => (w, h)
    };

    let display = Display::new(img, width, height);

    print!("{}", display.render());
}
