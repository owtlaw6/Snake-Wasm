use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};


pub struct Canvas {
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attribute_id: &str, width: u32, height: u32) -> Canvas {
        let window = window().expect("No global `window` exists");
        let document = window.document().expect("Should have a document on window");
        let canvas = document
            .query_selector(attribute_id)
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        Canvas {
            canvas,
            context,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.width);
        assert!(y < self.height);

        #[allow(deprecated)]
        self.context.set_fill_style(&JsValue::from_str(color));


        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.context.fill_rect(
            x as f64,
            y as f64,
            self.scaled_width as f64,
            self.scaled_height as f64,
        );
    }

    pub fn clear_all(&self) {
        #[allow(deprecated)]
        self.context.set_fill_style(&JsValue::from_str("white"));

        self.context.fill_rect(
            0.0,
            0.0,
            (self.width * self.scaled_width) as f64,
            (self.height * self.scaled_height) as f64,
        );
    }
}
