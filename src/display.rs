extern crate image;

extern crate test;

#[cfg(test)]
use test::Bencher;

use self::image::imageops::resize;

use braille::make_braille;

pub struct Display {
    image: image::GreyImage,
    pub char_width: u32,
    pub char_height: u32,
}

impl Display {
    pub fn new(img: image::GreyImage, width: u32, height: u32) -> Display {
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
                target.push(make_braille(dot_map));
            }
            target.push('\n');
        }
        target
    }
    fn sample(&self, x: u32, y: u32) -> bool {
        self.image[(x, y)][0] < 128
    }
}

#[bench]
fn bench_display(b: &mut Bencher) {
    let display = Display::new(&Path::new("image.png"), 80, 29);
    b.iter(|| display.render())
}
