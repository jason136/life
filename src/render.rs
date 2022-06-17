use wasm_bindgen::prelude::*;
use crate::{
    Node, 
    OptionExt,
};
use std::sync::Arc;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Renderer {
    canvas_width: i32,
    canvas_height: i32,
    offset_x: i32,
    offset_y: i32,
    border_width: i32,
    cell_width: i32,
    pixel_ratio: f32,
    image_data_pixels: Vec<u32>,
    image_data_bytes: Vec<u8>,
}

use std::sync::atomic::{AtomicUsize, Ordering};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[wasm_bindgen]
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            canvas_width: 0,
            canvas_height: 0,
            offset_x: 0,
            offset_y: 0,
            border_width: 0,
            cell_width: 20,
            pixel_ratio: 1.0,
            image_data_pixels: vec![],
            image_data_bytes: vec![],
        }
    }

    pub fn zoom(&mut self, out: bool, center_x: i32, center_y: i32) {
        if out {
            self.offset_x -= (self.offset_x - center_x) / 2;
            self.offset_y -= (self.offset_y - center_y) / 2;

            self.cell_width /= 2;
        }
        else {
            self.offset_x += (self.offset_x - center_x) / 2;
            self.offset_y += (self.offset_y - center_y) / 2;

            self.cell_width *= 2;
        }
    }
    pub fn center_view(&mut self) {
        self.offset_x = self.canvas_width >> 1;
        self.offset_y = self.canvas_height >> 1;
    }
    pub fn move_offset(&mut self, x: i32, y: i32) {
        self.offset_x += (x as f32 * self.pixel_ratio) as i32;
        self.offset_y += (y as f32 * self.pixel_ratio) as i32;
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.canvas_width = width;
        self.canvas_height = height;
    }
    pub fn set_center(&mut self, x: i32, y: i32) {
        self.offset_x = x;
        self.offset_y = y;
    }
    pub fn set_cell_width(&mut self, width: i32) {
        self.cell_width = width;
    }

    fn draw_cell(&mut self, x: i32, y: i32, set: bool) {
        let cell_x = x * self.cell_width + self.offset_x;
        let cell_y = y * self.cell_width + self.offset_y;
        let width = self.cell_width - (self.cell_width * self.border_width | 0 as i32);

        if set {
            // TODO call draw_square with a way to set the color
        }
    }

    fn draw_square(&mut self, mut x: i32, mut y: i32, size: i32) {

        let mut width = size - self.border_width;
        let mut height = width;

        if x < 0 {
            width += x;
            x = 0;
        }
        if x + width > self.canvas_width {
            width = self.canvas_width - x;
        }

        if y < 0 {
            height += y;
            y = 0;
        }
        if y + height > self.canvas_height {
            height = self.canvas_height - y;
        }
    
        if width <= 0 || height <= 0 {
            return;
        }

        let mut pointer = x + y * self.canvas_width;
        let row_width = self.canvas_width - width;

        for _ in 0..height {
            for _ in 0..width {
                self.image_data_pixels[pointer as usize] = 0xFFFFFFFF;
                log(format!("{}", pointer).as_str());
                pointer += 1;

                CALL_COUNT.fetch_add(1, Ordering::SeqCst);
            }
            pointer += row_width;
        }
    }

    fn draw_node(&mut self, node: Option<Arc<Node>>, mut size: i32, left: i32, top: i32) {
        if node.population() == 0 { return };

        if left + size + self.offset_x < 0 ||
            top + size + self.offset_y < 0 ||
            left + self.offset_x >= self.canvas_width ||
            top + self.offset_y >= self.canvas_height { return };
        

        if size <= 1 {
            if node.population() > 0 {
                self.draw_square(left + self.offset_x | 0, top + self.offset_y | 0, 1);
            }
        }
        else if node.level() == 0 {
            if node.population() > 0 {
                self.draw_square(left + self.offset_x, top + self.offset_y, self.cell_width);
            }
        }
        else {
            size /= 2;

            self.draw_node(node.a(), size, left, top);
            self.draw_node(node.b(), size, left + size, top);
            self.draw_node(node.c(), size, left, top + size);
            self.draw_node(node.d(), size, left + size, top + size);
        }
    }

    pub fn get_image_data(&mut self, node: &Node) -> *const u8 {
        self.image_data_pixels = vec![0x660000FF; (self.canvas_width * self.canvas_height) as usize];

        self.border_width = self.border_width * self.cell_width | 0 as i32;
        
        let size = 2_i32.pow(node.level() as u32 - 1) * self.cell_width;

        CALL_COUNT.store(0, Ordering::SeqCst);
        self.draw_node(Some(Arc::new(node.clone())), size, -size, -size);
        log(CALL_COUNT.load(Ordering::SeqCst).to_string().as_str());

        self.image_data_bytes = self.image_data_pixels.iter().flat_map(|val| val.to_be_bytes()).collect();

        self.image_data_bytes.as_ptr()
    }
}