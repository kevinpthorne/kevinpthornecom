use crate::bitset::Bitset;

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
const A: FontGlyph = 0b0000000_0010001010011100101001010;
const C: FontGlyph = 0b0000000_0111010001000011000101110;
const D: FontGlyph = 0b0000000_0011101001100010100100111;
const E: FontGlyph = 0b0000000_1111100001111110000111111;
const H: FontGlyph = 0b0000000_1000110001111111000110001;
const I: FontGlyph = 0b0000000_1111100100001000010011111;
const K: FontGlyph = 0b0000000_0100100101000110010101001;
const N: FontGlyph = 0b0000000_1000111001101011001110001;
const O: FontGlyph = 0b0000000_0111010001100011000101110;
const P: FontGlyph = 0b0000000_0000100001011111000101111;
const R: FontGlyph = 0b0000000_1100101001001110100100111;
const S: FontGlyph = 0b0000000_0111110000011100000111110;
const T: FontGlyph = 0b0000000_0010000100001000010011111;
const V: FontGlyph = 0b0000000_0010001010100011000110001;
const UNKNOWN: FontGlyph = 0b0000000_1010101010101010101010101;
const EMPTY: FontGlyph =   0b0000000_0000000000000000000000000;

pub const GLYPH_LEN: usize = 32 - 7;
pub const GLYPH_SIZE: usize = 5;
pub const KERNING: usize = 1;
// const FONT: Vec<FontGlyph> = vec![A, D, V];

pub fn get_glyph(c: char) -> FontGlyph {
    match c {
        'A' => A,
        'C' => C,
        'D' => D,
        'E' => E,
        'H' => H,
        'I' => I,
        'K' => K,
        'N' => N,
        'O' => O,
        'P' => P,
        'R' => R,
        'S' => S,
        'T' => T,
        'V' => V,
        ' ' => EMPTY,
        _ => UNKNOWN,
    }
}

pub fn get_glyphs(s: String) -> Vec<FontGlyph> {
    s.chars().map(|c| get_glyph(c)).collect()
}
