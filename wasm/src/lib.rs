mod bitset;
mod pixels;
mod font;
mod ui;
mod utils;

use font::GLYPH_SIZE;
use pixels::PixelBuffer;
use pixels::GREEN;
use pixels::RED;
use pixels::BLUE;
use pixels::WHITE;
use ui::is_point_in_rect;
use ui::Button;
use ui::Gesture;
use ui::GestureHandler;
use ui::HCenter;
use ui::Positioned;
use ui::Rectangle;
use ui::Renderable;
use ui::Text;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::MouseEvent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    // #[wasm_bindgen(js_namespace = console, js_name = log)]
    // fn log_u32(a: u32);

    fn alert(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    log("hello world!");
    alert("Hello, kevinpthornecom_wasm!!!");
}

const FPS: u32 = 15;

#[wasm_bindgen]
pub struct CanvasApp {
    canvas: web_sys::HtmlCanvasElement,
    screenbuff: PixelBuffer,
    tick: usize,
    last_frame_time: u32,
    ui_elements: Vec<Box<dyn Renderable>>,
    // todo fix
    gesturehandler_ui_elements: Vec<Box<dyn GestureHandler>>,
}

#[wasm_bindgen]
impl CanvasApp {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<CanvasApp, JsValue> {
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as usize;
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as usize;
        
        // TODO rip this out of the constructor
        let test_text: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 !@#$%^&*()[]{}\\|;':\",./<>?-=_+`~".to_string();
        let test_2: String = "THE QUICK, BROWN FOX JUMPS OVER THE LAZY DOG.".to_string();
        let instance = Self {
            canvas,
            screenbuff: PixelBuffer::new(width, height),
            tick: 0,
            last_frame_time: 0,
            ui_elements: vec![
                Box::new(HCenter::new((0, 10), Box::new(Text::new("KEVIN P. THORNE".to_string(), 4, WHITE)))),
                Box::new(Positioned::new((0, 100), Box::new(Text::new(test_text.clone().to_string(), 1, GREEN)))),
                Box::new(Positioned::new((0, 110), Box::new(Text::new(test_text.clone().to_string(), 2, RED)))),
                Box::new(Positioned::new((0, 125), Box::new(Text::new(test_text.clone().to_string(), 3, BLUE)))),
                Box::new(Positioned::new((0, 145), Box::new(Text::new(test_text.clone().to_string(), 4, GREEN)))),
                Box::new(Positioned::new((0, 170), Box::new(Text::new(test_2.clone().to_string(), 3, GREEN)))),
                Box::new(Positioned::new((10, 220), Box::new(Rectangle::new((10, 10), WHITE)))),
            ],
            gesturehandler_ui_elements: vec![
                Box::new(Button::new((10, 190), "BOOP".to_string(), 3, (120, 120, 120, 255), WHITE)),
            ],
        };
        log("canvas app loaded");
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
        let event_point: (usize, usize) = (event.page_x() as usize, event.page_y() as usize);
        for gesture_handler in self.gesturehandler_ui_elements.iter_mut().rev() {
            if is_point_in_rect(event_point, gesture_handler.get_collision_rect()) {
                console_log!("clicked on something! {:?}", event.type_());
                if let Some(gesture) = Gesture::of(event.clone()) {
                    gesture_handler.on_event(gesture);
                }
            }
        }
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
        let tick_str: String = self.tick.to_string();
        let frametime_str: String = delta_time.to_string();
        let stats_str = format!("FRAME {tick_str} . FRAMETIME {frametime_str}");

        // fancy gradient background
        for y in 0..self.screenbuff.height {
            for x in 0..self.screenbuff.width {
                let blue = (255.0 * ((1.0 / 255.0) * 3.14 * self.tick as f32).sin() + 255.0) as u8;
                self.screenbuff.set((x, y), (0, 0, blue, 255));
            }
        }

        // render UI elements
        for e in &mut self.ui_elements {
            e.render(&mut self.screenbuff);
        }
        for e in &mut self.gesturehandler_ui_elements {
            e.render(&mut self.screenbuff);
        }
        self.screenbuff.render_text(&stats_str, (0, self.screenbuff.height - (GLYPH_SIZE * 2)), GREEN, 2);

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
