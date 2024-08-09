use web_sys::MouseEvent;

use crate::{
    bitset::Bitset, pixels::{darken, Color, PixelBuffer}, text::{get_glyph, GLYPH_SIZE, KERNING}
};

/// x, y
pub type Point = (usize, usize);
/// topleft, bottomright
pub type Rect = (Point, Point);

pub fn is_point_in_rect((x, y): Point, rect: Rect) -> bool {
    let (topleft, bottomright) = rect;
    let (topleft_x, topleft_y) = topleft;
    let (bottomright_x, bottomright_y) = bottomright;

    x >= topleft_x && y >= topleft_y && x <= bottomright_x && y <=bottomright_y
}

pub trait Renderable {
    fn render(&mut self, buffer: &mut PixelBuffer) -> ();
}
pub enum Gesture {
    MouseDown,
    MouseUp
}
impl Gesture {
    pub fn of(event: MouseEvent) -> Option<Gesture> {
        match event.type_().as_str() {
            "mousedown" => Some(Self::MouseDown),
            "mouseup" => Some(Self::MouseUp),
            _ => {
                // console_log!("{:?}", event.type_());
                None
            },
        }
    }
}
pub trait GestureHandler: Renderable {
    fn on_event(&mut self, type_: Gesture) -> ();
    fn get_collision_rect(&self) -> Rect;
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
    fn render(&mut self, buffer: &mut PixelBuffer) {
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
    fn render(&mut self, buffer: &mut PixelBuffer) -> () {
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
    is_clicked: bool,
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
            is_clicked: false,
        }
    }
}
impl Renderable for Button {
    fn render(&mut self, buffer: &mut PixelBuffer) -> () {
        if self.is_clicked {
            self.rectangle.color = darken(self.color, 2);
            self.text.color = darken(self.text_color, 2);
        } else {
            self.rectangle.color = self.color;
            self.text.color = self.text_color;
        }
        self.rectangle.render(buffer);
        self.text.render(buffer);
    }
}
impl GestureHandler for Button {
    fn on_event(&mut self, type_: Gesture) -> () {
        match type_ {
            Gesture::MouseDown => self.is_clicked = true,
            _ => self.is_clicked = false,
        }
    }
    fn get_collision_rect(&self) -> Rect {
        self.rectangle.rect
    }
}