use std::char;

pub trait ToBraille {
    /// Get the unicode braille symbol for any given map of bits
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(0b10011101.to_braille() == '⢵');
    /// ```
    fn to_braille(&self) -> char;
}

impl ToBraille for u8 {
    fn to_braille(&self) -> char {
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
            (self >> 7 & 0b00000001u8) | // Moving X_______ to _______X
            (self >> 3 & 0b00001000u8) | // Moving _X______ to ____X___
            (self >> 4 & 0b00000010u8) | // Moving __X_____ to ______X_
            (self >> 0 & 0b00010000u8) | // Moving ___X____ to ___X____
            (self >> 1 & 0b00000100u8) | // Moving ____X___ to _____X__
            (self << 3 & 0b00100000u8) | // Moving _____X__ to __X_____
            (self << 5 & 0b01000000u8) | // Moving ______X_ to _X______
            (self << 7 & 0b10000000u8);  // Moving _______X to X_______
        char::from_u32(10240 + warped_dotmap as u32).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::ToBraille;

    #[test]
    fn test_u8_to_braille() {
        assert_eq!(0b00000000.to_braille(), '⠀');
        assert_eq!(0b11111111.to_braille(), '⣿');

        assert_eq!(0b00001111.to_braille(), '⣤');
        assert_eq!(0b11110000.to_braille(), '⠛');
        assert_eq!(0b10011001.to_braille(), '⢕');
        assert_eq!(0b10101010.to_braille(), '⡇');
        assert_eq!(0b01010101.to_braille(), '⢸');

        assert_eq!(0b10011101.to_braille(), '⢵');
    }
}
