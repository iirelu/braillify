#![allow(unstable)]

extern crate image;

use display::Display;

mod display;
mod braille;

enum Args {
    NoSize(image::GreyImage),
    WithSize(image::GreyImage, (u32, u32)),
}

fn main() {
    let usage = "braillify image [size]\n\
                 \timage: Any image (example: image.png)\n\
                 \tsize: The desired output size. (example: 50x25)\n\
                 \t\tIf a size isn't given, it'll be guessed automatically.";
    match parse_args(std::os::args()) {
        Ok(args) => {
            let display = match args {
                Args::NoSize(img) => {
                    let (width, height) = img.dimensions();
                    Display::new(img, width/10, height/20)
                },
                Args::WithSize(img, (width, height)) => {
                    Display::new(img, width, height)
                },
            };
            println!("{}", display.render());
        },
        Err(err) => println!("{}\n\nUsage: {}", err, usage)
    };

}

fn parse_args(args: Vec<String>) -> Result<Args, &'static str> {
    if args.len() < 2 {
        return Err("Too few arguments!");
    }

    let img = match image::open(&Path::new(&args[1])) {
        Ok(image) => image.to_luma(),
        Err(_) => return Err("Couldn't open image!")
    };

    match args.len() {
        2 => Ok(Args::NoSize(img)),
        3 => Ok(Args::WithSize(img, match parse_size(args[2].as_slice()) {
            Some(x) => x,
            None => return Err("Couldn't parse size!")
        })),
        _ => Err("Bad arguments!")
    }
}

/// Parses something that looks like a size (50x50, etc)
///
/// Returns None if it can't be parsed
fn parse_size(size: &str) -> Option<(u32, u32)> {
    let splitted = size.splitn(1, 'x') // Only want at most two slices
        .filter_map(|s| s.parse::<u32>())
        .collect::<Vec<_>>();

    match splitted.len() {
        2 => Some((splitted[0], splitted[1])),
        _ => None
    }
}
