pub type FontGlyph = u32;

/// Glyphs are a 5x5 grid
/// 
/// most significant 7 bits are currently unused.
/// 
/// Layout
/// ----------------
/// |24|23|22|21|20|
/// |19|18|17|16|15|
/// |14|13|12|11|10|
/// | 9| 8| 7| 6| 5|
/// | 4| 3| 2| 1| 0|
/// ----------------
///                            0-----------------------24
const A: FontGlyph = 0b0000000_1000110001111111000101110;
const B: FontGlyph = 0b0000000_0111110001011111000101111;
const C: FontGlyph = 0b0000000_0111010001000011000101110;
const D: FontGlyph = 0b0000000_0011101001100010100100111;
const E: FontGlyph = 0b0000000_1111100001111110000111111;
const F: FontGlyph = 0b0000000_0001000010111110001011111;
const G: FontGlyph = 0b0000000_0111010001111010000101110;
const H: FontGlyph = 0b0000000_1000110001111111000110001;
const I: FontGlyph = 0b0000000_1111100100001000010011111;
const J: FontGlyph = 0b0000000_0011001001010000100011111;
const K: FontGlyph = 0b0000000_0100100101000110010101001;
const L: FontGlyph = 0b0000000_1111100001000010000100001;
const M: FontGlyph = 0b0000000_1000110001101011101110001;
const N: FontGlyph = 0b0000000_1000111001101011001110001;
const O: FontGlyph = 0b0000000_0111010001100011000101110;
const P: FontGlyph = 0b0000000_0000100001011111000101111;
const Q: FontGlyph = 0b0000000_1011001001100011000101110;
const R: FontGlyph = 0b0000000_1100101001001110100100111;
const S: FontGlyph = 0b0000000_0111110000011100000111110;
const T: FontGlyph = 0b0000000_0010000100001000010011111;
const U: FontGlyph = 0b0000000_0111010001100011000110001;
const V: FontGlyph = 0b0000000_0010001010100011000110001;
const W: FontGlyph = 0b0000000_0101010101100011000110001;
const X: FontGlyph = 0b0000000_1000101010001000101010001;
const Y: FontGlyph = 0b0000000_0010000100001000101010001;
const Z: FontGlyph = 0b0000000_1111100001011101000011111;
const ONE: FontGlyph =      0b0000000_1111100100001000011000100;
const TWO: FontGlyph =      0b0000000_1111100001111111000011111;
const THREE: FontGlyph =    0b0000000_0111110000111101000001111;
const FOUR: FontGlyph =     0b0000000_1000010000111111000110001;
const FIVE: FontGlyph =     0b0000000_1111110000111110000111111;
const SIX: FontGlyph =      0b0000000_0111010001011110000101110;
const SEVEN: FontGlyph =    0b0000000_0010000100010001000011111;
const EIGHT: FontGlyph =    0b0000000_0111010001011101000101110;
const NINE: FontGlyph =     0b0000000_0111010000111101000101110;
const ZERO: FontGlyph =     0b0000000_0111010011101011100101110;
const UNKNOWN: FontGlyph = 0b0000000_1010101010101010101010101;
const EMPTY: FontGlyph =   0b0000000_0000000000000000000000000;

pub const GLYPH_LEN: usize = 32 - 7;
pub const GLYPH_SIZE: usize = 5;
pub const KERNING: usize = 1;
// const FONT: Vec<FontGlyph> = vec![A, D, V];

pub fn get_glyph(c: char) -> FontGlyph {
    match c {
        'A' => A,
        'B' => B,
        'C' => C,
        'D' => D,
        'E' => E,
        'F' => F,
        'G' => G,
        'H' => H,
        'I' => I,
        'J' => J,
        'K' => K,
        'L' => L,
        'M' => M,
        'N' => N,
        'O' => O,
        'P' => P,
        'Q' => Q,
        'R' => R,
        'S' => S,
        'T' => T,
        'U' => U,
        'V' => V,
        'W' => W,
        'X' => X,
        'Y' => Y,
        'Z' => Z,
        '1' => ONE,
        '2' => TWO,
        '3' => THREE,
        '4' => FOUR,
        '5' => FIVE,
        '6' => SIX,
        '7' => SEVEN,
        '8' => EIGHT,
        '9' => NINE,
        '0' => ZERO,
        ' ' => EMPTY,
        _ => UNKNOWN,
    }
}

pub fn get_glyphs(s: String) -> Vec<FontGlyph> {
    s.chars().map(|c| get_glyph(c)).collect()
}
