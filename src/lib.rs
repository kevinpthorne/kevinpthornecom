mod bitset;
mod utils;
mod text;

use std::cell::RefCell;
use std::rc::Rc;

use text::get_glyphs;
use text::render_glyphs;
use text::FontGlyph;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log("hello world!");
    alert("Hello, kevinpthornecom_wasm!!!");
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .expect("should have `window`")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub struct CanvasApp {
    canvas: web_sys::HtmlCanvasElement,
}

#[wasm_bindgen]
impl CanvasApp {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<CanvasApp, JsValue> {
        Ok(CanvasApp { canvas })
    }

    pub fn start_animation(&mut self) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let width = self.canvas.width() as usize;
        let height = self.canvas.height() as usize;
        let ctx = self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut cursor_x = 0 as usize;
        let mut cursor_y = 0 as usize;
        let text: String = "DVD".to_string();
        let text_bitmap: Vec<u8> = render_glyphs(&text);

        let mut data = vec![0 as u8; width * height * 4]; // RGBA data
        let mut tick = 0 as usize;
        *g.borrow_mut() = Some(Closure::new(move || {
            if tick > 30000 {
                log("all done");

                // Drop our handle to this closure so that it will get cleaned
                // up once we return.
                let _ = f.borrow_mut().take();
                return;
            }

            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) * 4;
                    let red = i;
                    let green = i + 1;
                    let blue = i + 2;
                    let alpha = i + 3;
                    data[blue] = (255.0 * ((1.0 / 255.0) * 3.14 * tick as f32).sin() + 255.0) as u8;
                    // if y <= 5 && x <= 5 {
                    //     data[red] = 255;
                    // }
                    data[alpha] = 255; 
                }
            }
            
            for byte in text_bitmap.iter() {
                for bit in 0..8 { // Iterate over all 8 bits in a byte (representing 2 rows of the glyph)
                  let mask = 1 << (7 - bit);
                  let set_pixel = (mask & *byte) != 0; // Check if the bit is set
            
                  if set_pixel {
                    let pixel_index = (cursor_y * width + cursor_x) * 4;
            
                    // Ensure we stay within screen bounds
                    if pixel_index < data.len() && cursor_x < width {
                      // Set all channels to 255 for white color (RGBA)
                      data[pixel_index] = 255;
                      data[pixel_index + 1] = 255;
                      data[pixel_index + 2] = 255;
                      data[pixel_index + 3] = 255;
                    }
            
                    // Move cursor position based on bit position (every even bit moves to the next row)
                    cursor_x += (bit % 2) as usize;
                  }
            
                  // After processing 2 pixels (1 bit per row), move to the next row within the glyph
                  if bit % 2 == 1 {
                    cursor_y += 1;
                    cursor_x = 0; // Reset cursor X for the next character
                  }
                }
            
                // Move cursor to the next character position (5 pixels wide)
                cursor_x += 5;
            
                // Handle wrapping around at the end of a line
                if cursor_x >= width {
                  cursor_x = 0;
                  cursor_y += 1;
            
                  // Check for going off-screen at the bottom
                  if cursor_y >= height {
                    break;
                  }
                }
              }

            let clamped_data = wasm_bindgen::Clamped(data.as_ref());
            let image_data = web_sys::ImageData::new_with_u8_clamped_array(clamped_data, width as u32).unwrap();
            let _ = ctx.put_image_data(&image_data, 0.0, 0.0);
            
            tick += 1;

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }

}

#[wasm_bindgen]
pub fn start(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    log("starting canvas");

    let mut app = CanvasApp::new(canvas)?;
    let _ = app.start_animation();

    log("canvas drawn!");

    Ok(())
}
