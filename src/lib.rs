mod bitset;
mod pixels;
mod utils;
mod text;

use std::cell::RefCell;
use std::rc::Rc;

use bitset::Bitset;
use pixels::PixelBuffer;
use pixels::GREEN;
use text::get_glyph;
use text::get_glyphs;
use text::render_glyphs;
use text::FontGlyph;
use text::GLYPH_SIZE;
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
        // let text_bitmap: Bitset = render_glyphs(&text, 1);

        let mut data = vec![0 as u8; width * height * 4]; // RGBA data
        let mut screenbuff = PixelBuffer::new(width, height);
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
                    let blue = (255.0 * ((1.0 / 255.0) * 3.14 * tick as f32).sin() + 255.0) as u8;
                    screenbuff.set((x, y), (0, 0, blue, 255));
                }
            }

            // render text
            screenbuff.render_text(&text, (100, 100), GREEN);
            
            let clamped_data = wasm_bindgen::Clamped(screenbuff.data_as_ref());
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
