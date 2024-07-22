use crate::{bitset::Bitset, text::{get_glyph, GLYPH_SIZE, KERNING}};


pub type Point = (usize, usize);
pub type Color = (u8, u8, u8, u8);

pub const RED: Color = (255, 0, 0, 255);
pub const BLUE: Color = (0, 0, 255, 255);
pub const GREEN: Color = (0, 255, 0, 255);

pub struct PixelBuffer {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

pub const NUM_CHANNELS: usize = 4;

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            data: vec![0 as u8; width * height * NUM_CHANNELS],
            width,
            height,
        }
    }

    pub fn set(&mut self, (x, y): Point, (r, g, b, a): Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = (y * self.width + x) * NUM_CHANNELS;
        let red = offset;
        let green = offset + 1;
        let blue = offset + 2;
        let alpha = offset + 3;

        self.data[red] = r;
        self.data[green] = g;
        self.data[blue] = b;
        self.data[alpha] = a;
    }

    pub fn render_text(&mut self, text: &String, (screen_x, screen_y): Point, color: Color, scale: usize) {
        for (i, c) in text.chars().enumerate() {
            let glyph = get_glyph(c);
            let glyph_bitmap = Bitset::from_u32(glyph);
            for y in 0..GLYPH_SIZE {
                for x in 0..GLYPH_SIZE {
                    let glyph_offset = y * GLYPH_SIZE + x;

                    for scale_offset_y in 0..scale {
                        for scale_offset_x in 0..scale {
                            let screen_y_offset = screen_y + (y * scale) + scale_offset_y;
                            let screen_x_offset = screen_x + (x * scale) + (i * GLYPH_SIZE * scale) + (i * KERNING) + scale_offset_x;
                            if glyph_bitmap.get(glyph_offset) {
                                self.set((screen_x_offset, screen_y_offset), color);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn data_as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}