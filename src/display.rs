extern crate image;

use self::image::imageops::resize;

use braille::make_braille;

pub struct Display {
    image: image::GrayImage,
    pub char_width: u32,
    pub char_height: u32,
}

impl Display {
    pub fn new(img: image::GrayImage, width: u32, height: u32) -> Display {
        Display {
            image: resize(&img, width*2, height*4, image::Lanczos3),
            char_width: width,
            char_height: height,
        }
    }

    pub fn render(&self) -> String {
        let mut target = String::with_capacity(
            // The char_height is added to account for newlines
            (self.char_height*self.char_width + self.char_height) as usize
        );
        for y in 0..self.char_height {
            for x in 0..self.char_width {
                target.push(self.braillify_block(x, y));
            }
            target.push('\n');
        }
        target
    }

    fn braillify_block(&self, x: u32, y: u32) -> char {
        let mut dot_map = 0b0000_0000;
        for i in 0..8 {
            let abs_x = (x*2) + (i % 2);
            let abs_y = (y*4) + (i / 2);

            dot_map |= if self.sample(abs_x, abs_y) {
                0b1000_0000 >> i
            } else {
                0
            };
        }
        make_braille(dot_map)
    }

    fn sample(&self, x: u32, y: u32) -> bool {
        self.image[(x, y)][0] < 128
    }
}

#[cfg(test)]
mod tests {
    use super::Display;
    use super::image;

    #[test]
    fn test_overall_functionality() {
        let image = image::open("tests/rust-logo.png").unwrap().to_luma();
        let display = Display::new(image, 20, 10);

        let expected = "\
⠀⠀⠀⠀⠀⠀⣤⣰⣦⣴⣦⣴⣄⣤⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣤⣼⣿⡿⠟⠛⢧⡼⠛⠻⢿⣿⣧⡤⠀⠀⠀
⠀⠀⣶⣿⣿⣥⣤⣤⣤⣤⣤⣤⣄⣀⠉⢿⣷⡶⠀⠀
⠀⣻⡿⢿⡿⣿⣿⣿⡿⠿⠿⢿⣿⣿⡧⢠⠟⢿⣏⠀
⢈⣿⣷⠚⠁⣿⣿⣿⣷⣶⣶⣾⣿⡿⠃⠈⠓⣿⣟⡁
⢈⣿⣿⠀⠀⣿⣿⣿⡏⠉⠙⢿⣿⣿⡆⠀⣶⣿⣯⡁
⠀⣽⣿⣿⣿⣿⣿⣿⣿⣿⠀⠸⣿⣿⣿⣿⣿⣿⣏⠀
⠀⠀⠿⣿⣧⠴⣆⠀⠀⠀⠀⠀⠀⡴⢤⣾⡿⠷⠀⠀
⠀⠀⠀⠛⢻⣶⣿⣶⣤⣤⣤⣤⣶⣷⣾⡟⠓⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠛⠙⠟⠻⠟⠻⠋⠛⠀⠀⠀⠀⠀⠀
";

        assert!(display.sample(0, 0) == false);
        assert!(display.sample(4, 19) == true);

        assert!(display.braillify_block(0, 5) == '⠀');
        assert!(display.braillify_block(0, 6) == '⣤');
        assert!(display.braillify_block(0, 7) == '⣰');

        assert!(display.render() == expected);
    }
}
