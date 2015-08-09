use std::char;

/// Get the unicode braille symbol for any given map of bits
///
/// # Arguments
///
/// `dotmap` - A u8 that gets its bits used as a mask for which dots on the
/// braille symbol should be "lit".
///
/// # Examples
///
/// ```
/// assert!(make_braille(0b10011101) == '⢵');
/// ```
pub fn make_braille(dotmap: u8) -> char {
    // Braille in unicode works weirdly so we have to warp the bits
    // around.
    //
    // If you number each bit of a byte like 0b12345678 then a braille
    // character looks like:
    //
    //      8  5
    //      7  4
    //      6  3
    //      2  1
    //
    // This is the core reason for all the weird bitwise reordering.
    //
    // See https://en.wikipedia.org/wiki/Braille_Patterns for more info
    let warped_dotmap: u8 =
        (dotmap >> 7 & 0b00000001) | // Moving X_______ to _______X
        (dotmap >> 3 & 0b00001000) | // Moving _X______ to ____X___
        (dotmap >> 4 & 0b00000010) | // Moving __X_____ to ______X_
        (dotmap >> 0 & 0b00010000) | // Moving ___X____ to ___X____
        (dotmap >> 1 & 0b00000100) | // Moving ____X___ to _____X__
        (dotmap << 3 & 0b00100000) | // Moving _____X__ to __X_____
        (dotmap << 5 & 0b01000000) | // Moving ______X_ to _X______
        (dotmap << 7 & 0b10000000);  // Moving _______X to X_______
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
