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
const A: FontGlyph = 0b0000000_0010001010011100101001010;
const D: FontGlyph = 0b0000000_0011101001100010100100111;
const V: FontGlyph = 0b0000000_0010001010100011000110001;
const UNKNOWN: FontGlyph = 0b0000000_1010101010101010101010101;

pub const GLYPH_LEN: usize = 32 - 7;
pub const GLYPH_SIZE: usize = 5;
// const FONT: Vec<FontGlyph> = vec![A, D, V];

pub fn get_glyph(c: char) -> FontGlyph {
    match c {
        'A' => A,
        'D' => D,
        'V' => V,
        _ => UNKNOWN,
    }
}

pub fn get_glyphs(s: String) -> Vec<FontGlyph> {
    s.chars().map(|c| get_glyph(c)).collect()
}

// pub fn get_scanlines(s: String) -> Vec<u32> {
//     let glyphs = get_glyphs(s);
//     let mut line_chunks = vec![0 as u32];
//     let mut result = vec![0 as u32];
//     for y in 0..5 {
//         let line = 0 as u32;
//         for x in 0..5 {
//             let mask = 0b11111 << ((5 - x) * 5);
//             for c in &glyphs {
//                 line_chunks.push(c ^ mask);
//             }
//         }
//         result.push(line);
//     }

//     for y
//     result
// }

pub fn render_glyphs(text: &str, size: usize) -> Bitset {
    // TODO size is unused
    let mut bitset = Bitset::new(GLYPH_LEN * size * text.len());
    let glyphs: Vec<Bitset> = text.chars().map(|c| Bitset::from_u32(get_glyph(c))).collect();
    let mut j: usize = 0;
    for row in 0..GLYPH_SIZE {
        for i in 0..(text.len()) {
            let glyph = &glyphs[i];
            for col in 0..GLYPH_SIZE {
                let bitmap_offset = j;
                let glyph_offset = (row * GLYPH_SIZE) + col;
                bitset.set(bitmap_offset, glyph.get(glyph_offset));
                j += 1;
                // TODO glyphs may be reflected
            }
        }
    }

    bitset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        render_glyphs("DV", 1);
    }
}