use web_sys::MouseEvent;

use crate::{
    bitset::Bitset, font::{get_glyph, GLYPH_SIZE, KERNING}, pixels::{darken, Color, PixelBuffer, RED}
};

/// x, y
pub type Point = (usize, usize);
/// topleft, bottomright
pub type Rect = (Point, Point);
/// width, height
pub type Size = Point;

pub fn is_point_in_rect((x, y): Point, rect: Rect) -> bool {
    let (topleft, bottomright) = rect;
    let (topleft_x, topleft_y) = topleft;
    let (bottomright_x, bottomright_y) = bottomright;

    x >= topleft_x && y >= topleft_y && x <= bottomright_x && y <=bottomright_y
}

pub trait Renderable {
    fn render(&mut self, buffer: &mut PixelBuffer) -> ();
}
pub trait Drawable {
    fn draw(&mut self, buffer: &mut PixelBuffer, pos: Point) -> ();
    fn get_render_size(&self) -> Size;
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

pub struct HCenter {
    child: Box<dyn Drawable>,
    pos: Point,
    debug_mode: bool
}
impl HCenter {
    pub fn new(pos: Point, child: Box<dyn Drawable>) -> Self {
        Self {
            pos,
            child,
            debug_mode: false,
        }
    }
}
impl Renderable for HCenter {
    fn render(&mut self, buffer: &mut PixelBuffer) -> () {
        // find center point, offset child
        let (child_width, _) = self.child.get_render_size();
        // let (_, height) = self.get_render_size(buffer);
        let center_x = self.pos.0 + (buffer.width / 2);
        let child_x = center_x - (child_width / 2);
        // self.child.pos = (0,0);
        self.child.draw(buffer, (child_x, self.pos.1));
        if self.debug_mode {
            buffer.set((center_x, self.pos.1), RED);
            buffer.set((buffer.width - 10, self.pos.1), RED);
        }
    }
}

pub struct Positioned {
    pos: Point,
    child: Box<dyn Drawable>,
}
impl Positioned {
    pub fn new(pos: Point, child: Box<dyn Drawable>) -> Self {
        Self {
            pos,
            child,
        }
    }
}
impl Renderable for Positioned {
    fn render(&mut self, buffer: &mut PixelBuffer) -> () {
        self.child.draw(buffer, self.pos);
    }
}

pub struct Text {
    text: String,
    scale: usize,
    color: Color,
}
impl Text {
    pub fn new(text: String, scale: usize, color: Color) -> Self {
        Self {
            text,
            scale,
            color,
        }
    }
}

impl Drawable for Text {
    fn draw(&mut self, buffer: &mut PixelBuffer, pos: Point) {
        for (i, c) in self.text.chars().enumerate() {
            let glyph = get_glyph(c);
            let glyph_bitmap = Bitset::from_u32(glyph); // TODO memoize
            for y in 0..GLYPH_SIZE {
                for x in 0..GLYPH_SIZE {
                    let glyph_offset = y * GLYPH_SIZE + x;

                    for scale_offset_y in 0..self.scale {
                        for scale_offset_x in 0..self.scale {
                            let screen_y_offset = pos.1 + (y * self.scale) + scale_offset_y;
                            let screen_x_offset = pos.0
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
    fn get_render_size(&self) -> Size {
        (
            GLYPH_SIZE * self.scale * self.text.len() + (KERNING * (self.text.len() - 1)), 
            GLYPH_SIZE * self.scale
        )
    }
}

pub struct Rectangle {
    size: Size,
    color: Color,
}
impl Rectangle {
    pub fn new(size: Size, color: Color) -> Self {
        Self { size, color }
    }
}
impl Drawable for Rectangle {
    fn draw(&mut self, buffer: &mut PixelBuffer, pos: Point) -> () {
        let (width, height) = self.size;
        for y in pos.1..(pos.1 + height) {
            for x in pos.0..(pos.0 + width) {
                buffer.set((x, y), self.color);
            }
        }
    }

    fn get_render_size(&self) -> Size {
        self.size
    }
}

pub struct Button {
    pos: Point,
    rectangle: Rectangle,
    text: Text,
    color: Color,
    text_color: Color,
    // selected_color: Color,
    // selected_text_color: Color,
    // is_selected: bool,
    is_clicked: bool,
    margin: usize,
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
        let margin = 5;
        let text = Text::new(text, scale, text_color);
        let rect_size = (
            text.get_render_size().0 + (margin * 2),
            text.get_render_size().1 + (margin * 2)
        );

        
        Self {
            pos,
            rectangle: Rectangle::new(rect_size, color),
            text,
            color,
            text_color,
            // selected_color,
            // selected_text_color,
            // is_selected: false,
            is_clicked: false,
            margin,
        }
    }
}
impl Renderable for Button {
    fn render(&mut self, buffer: &mut PixelBuffer) -> () {
        let text_pos: Point = (
            self.pos.0 + self.margin,
            self.pos.1 + self.margin,
        );

        if self.is_clicked {
            self.rectangle.color = darken(self.color, 2);
            self.text.color = darken(self.text_color, 2);
        } else {
            self.rectangle.color = self.color;
            self.text.color = self.text_color;
        }
        self.rectangle.draw(buffer, self.pos);
        self.text.draw(buffer, text_pos);
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
        let (x, y) = self.pos;
        let (w, h) = self.rectangle.size;
        (
            (x, y),
            (x + w, y + h)
        )
    }
}