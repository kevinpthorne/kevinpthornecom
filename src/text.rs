pub type FontGlyph = u32;

const A: FontGlyph = 0b0000000_0010001010011100101001010;
const D: FontGlyph = 0b0000000_1111010001100011000111110;
const V: FontGlyph = 0b0000000_1000110001100010101000100;
const UNKNOWN: FontGlyph = 0b0000000_1010101010101010101010101;

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

pub fn render_glyphs(text: &str) -> Vec<u8> {
    let mut bitmap: Vec<u8> = Vec::new();
    for c in text.chars() {
        let glyph_byte = get_glyph(c);
        for row in 0..5 {
            let mut row_bits = 0;
            for col in 0..5 {
                let mask = 1 << (7 - col);
                if (glyph_byte & mask) != 0 {
                    row_bits |= 1 << col;
                }
            }
            bitmap.push(row_bits);
        }
    }
    bitmap
}
