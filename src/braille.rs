use std::char;

/// Get the unicode braille symbol for any given map of bits
///
/// # Arguments
///
/// `dotmap` - A (usually) binary literal that's used to generate the unicode
/// symbol that gets returned.
///
/// # Examples
/// make_braille(0b10011101); // Returns ⢵
pub fn make_braille(dotmap: u8) -> char {
    // Braille in unicode works weirdly so we have to warp the values
    // around.
    // See https://en.wikipedia.org/wiki/Braille_Patterns for more info
    let warped_dotmap: u8 =
        (dotmap >> 7 & 0b00000001) | // Moving 10000000 to 00000001
        (dotmap >> 3 & 0b00001000) | // Moving 01000000 to 00001000
        (dotmap >> 4 & 0b00000010) | // Moving 00100000 to 00000010
        (dotmap >> 0 & 0b00010000) | // Moving 00010000 to 00010000
        (dotmap >> 1 & 0b00000100) | // Moving 00001000 to 00000100
        (dotmap << 3 & 0b00100000) | // Moving 00000100 to 00100000
        (dotmap << 5 & 0b01000000) | // Moving 00000010 to 01000000
        (dotmap << 7 & 0b10000000);  // Moving 00000001 to 10000000
    char::from_u32(10240 + warped_dotmap as u32).unwrap()
}


#[cfg(test)]
mod tests {
    use super::make_braille;

    #[test]
    fn test_make_braille() {
        assert!(make_braille(0b00000000) == '⠀');
        assert!(make_braille(0b11111111) == '⣿');

        assert!(make_braille(0b00001111) == '⣤');
        assert!(make_braille(0b11110000) == '⠛');
        assert!(make_braille(0b10011001) == '⢕');
        assert!(make_braille(0b10101010) == '⡇');
        assert!(make_braille(0b01010101) == '⢸');

        assert!(make_braille(0b10011101) == '⢵');
    }
}
