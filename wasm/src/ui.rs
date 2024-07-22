use crate::{
    bitset::Bitset,
    pixels::{Color, PixelBuffer},
    text::{get_glyph, GLYPH_SIZE, KERNING},
};

pub type Point = (usize, usize);
pub type Rect = (Point, Point);

pub trait Renderable {
    fn render(&self, buffer: &mut PixelBuffer) -> ();
}
pub trait Clickable {
    fn on_click(&self) -> ();
}

pub struct Text {
    pos: Point,
    text: String,
    scale: usize,
    color: Color,
}
impl Text {
    pub fn new(pos: Point, text: String, scale: usize, color: Color) -> Self {
        Self {
            pos,
            text,
            scale,
            color,
        }
    }
}

impl Renderable for Text {
    fn render(&self, buffer: &mut PixelBuffer) {
        for (i, c) in self.text.chars().enumerate() {
            let glyph = get_glyph(c);
            let glyph_bitmap = Bitset::from_u32(glyph); // TODO memoize
            for y in 0..GLYPH_SIZE {
                for x in 0..GLYPH_SIZE {
                    let glyph_offset = y * GLYPH_SIZE + x;

                    for scale_offset_y in 0..self.scale {
                        for scale_offset_x in 0..self.scale {
                            let screen_y_offset = self.pos.1 + (y * self.scale) + scale_offset_y;
                            let screen_x_offset = self.pos.0
                                + (x * self.scale)
                                + (i * GLYPH_SIZE * self.scale)
                                + (i * KERNING)
                                + scale_offset_x;
                            if glyph_bitmap.get(glyph_offset) {
                                buffer.set((screen_x_offset, screen_y_offset), self.color);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct Rectangle {
    rect: Rect,
    color: Color,
}
impl Rectangle {
    pub fn new(rect: Rect, color: Color) -> Self {
        Self { rect, color }
    }
}
impl Renderable for Rectangle {
    fn render(&self, buffer: &mut PixelBuffer) -> () {
        let ((topleft_x, topleft_y), (botright_x, botright_y)) = self.rect;
        for y in topleft_y..botright_y {
            for x in topleft_x..botright_x {
                buffer.set((x, y), self.color);
            }
        }
    }
}

pub struct Button {
    rectangle: Rectangle,
    text: Text,
    color: Color,
    text_color: Color,
    // selected_color: Color,
    // selected_text_color: Color,
    // is_selected: bool,
}
impl Button {
    pub fn new(
        pos: Point,
        text: String,
        scale: usize,
        color: Color,
        text_color: Color,
        // selected_color: Color,
        // selected_text_color: Color,
    ) -> Self {
        let margin: usize = 5;
        let rect: Rect = (
            pos,
            (
                pos.0 + (GLYPH_SIZE * text.len() * scale + KERNING * text.len()) + margin * 2,
                pos.1 + (GLYPH_SIZE * scale) + margin * 2,
            ),
        );
        let text_pos: Point = (
            pos.0 + margin,
            pos.1 + margin,
        );
        
        Self {
            rectangle: Rectangle::new(rect, color),
            text: Text::new(text_pos, text, scale, text_color),
            color,
            text_color,
            // selected_color,
            // selected_text_color,
            // is_selected: false,
        }
    }
}
impl Renderable for Button {
    fn render(&self, buffer: &mut PixelBuffer) -> () {
        self.rectangle.render(buffer);
        self.text.render(buffer);
    }
}
