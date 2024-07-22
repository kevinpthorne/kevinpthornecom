mod bitset;
mod pixels;
mod text;
mod ui;
mod utils;

use pixels::PixelBuffer;
use pixels::GREEN;
use pixels::WHITE;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::MouseEvent;

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

const FPS: u32 = 15;

#[wasm_bindgen]
pub struct CanvasApp {
    canvas: web_sys::HtmlCanvasElement,
    screenbuff: PixelBuffer,
    tick: usize,
    last_frame_time: u32,
}

#[wasm_bindgen]
impl CanvasApp {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<CanvasApp, JsValue> {
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as usize;
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as usize;
        let instance = Self {
            canvas,
            screenbuff: PixelBuffer::new(width, height),
            tick: 0,
            last_frame_time: 0,
        };
        Ok(instance)
    }

    pub fn on_resize(&mut self) {
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as usize;
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as usize;
        self.screenbuff = PixelBuffer::new(width, height);
        self.last_frame_time = 0;
    }

    pub fn on_click(&mut self, event: MouseEvent) {
        // log(event.page_x());
    }

    /// requestAnimationFrame usually calls as fast as the display is
    /// configured for (i.e. 60 or 120Hz). This regulates drawing to 
    /// [FPS]
    pub fn on_frame(&mut self, timestamp: u32) -> Result<(), JsValue> {
        let elapsed = timestamp - self.last_frame_time;
        if elapsed > (1000 / FPS) {
            let delta = elapsed - (1000 / FPS);
            let _ = self.render(delta);
            self.last_frame_time = timestamp;
            self.tick += 1;
        }
        Ok(())
    }

    fn render(&mut self, delta_time: u32) -> Result<(), JsValue> {
        let title: String = "KEVIN THORNE".to_string();
        let text: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 !@#$%^&*()[]{}\\|;':\",./<>?-=_+`~".to_string();
        let tick_str: String = self.tick.to_string();
        let frametime_str: String = delta_time.to_string();

        for y in 0..self.screenbuff.height {
            for x in 0..self.screenbuff.width {
                let blue = (255.0 * ((1.0 / 255.0) * 3.14 * self.tick as f32).sin() + 255.0) as u8;
                self.screenbuff.set((x, y), (0, 0, blue, 255));
                if y >= 100 && y < 105 && x >= 100 && x < 100 + 25 {
                    self.screenbuff.set((x, y), (0, 0, 0, 255));
                }
            }
        }

        // render text
        self.screenbuff.render_text(&title, (50, 50), WHITE, 4);
        self.screenbuff.render_text(&tick_str, (0, 0), GREEN, 2);
        self.screenbuff.render_text(&frametime_str, (0, 15), GREEN, 2);
        self.screenbuff.render_text(&text, (50, 100), GREEN, 1);
        self.screenbuff.render_text(&text, (50, 110), GREEN, 2);
        self.screenbuff.render_text(&text, (50, 125), GREEN, 3);
        self.screenbuff.render_text(&text, (50, 145), GREEN, 4);

        let clamped_data = wasm_bindgen::Clamped(self.screenbuff.data_as_ref());
        let image_data =
            web_sys::ImageData::new_with_u8_clamped_array(clamped_data, self.screenbuff.width as u32).unwrap();
        let _ = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
            .put_image_data(&image_data, 0.0, 0.0);

        // self.tick += 1;

        Ok(())
    }
}

#[wasm_bindgen]
pub fn init(canvas: web_sys::HtmlCanvasElement) -> Result<CanvasApp, JsValue> {
    Ok(CanvasApp::new(canvas)?)
}
