use wasm_bindgen::prelude::*;
use crate::{
    Node, 
    OptionExt,
};
use std::sync::Arc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Renderer {
    background_color: u32,
    cell_color: u32,
    added_cell_color: u32,

    canvas_width: i32,
    canvas_height: i32,
    canvas_offset_x: i32,
    canvas_offset_y: i32,

    border_width: f32,
    border_pixels: i32,
    cell_width: f32,
    pixel_ratio: f32,

    image_data_pixels: Vec<u32>,
    image_data_bytes: Vec<u8>,
    added_cells: Vec<(i32, i32, f32)>,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            background_color: 0x000000FF,
            cell_color: 0xFFFFFFFF,
            added_cell_color: 0xFF00FF00,

            canvas_width: 0,
            canvas_height: 0,
            canvas_offset_x: 0,
            canvas_offset_y: 0,

            border_width: 0.1,
            border_pixels: 0,
            cell_width: 32.0,
            pixel_ratio: 1.0,

            image_data_pixels: Vec::new(),
            image_data_bytes: Vec::new(),
            added_cells: Vec::new(),
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }
    pub fn set_cell_color(&mut self, color: u32) {
        self.cell_color = color;
    }

    pub fn zoom(&mut self, out: bool, center_x: i32, center_y: i32) {
        if out {
            self.canvas_offset_x -= (self.canvas_offset_x - center_x) / 2;
            self.canvas_offset_y -= (self.canvas_offset_y - center_y) / 2;

            self.cell_width /= 2.0;
        }
        else {
            self.canvas_offset_x += self.canvas_offset_x - center_x;
            self.canvas_offset_y += self.canvas_offset_y - center_y;

            self.cell_width *= 2.0;
        }
    }

    pub fn zoom_at(&mut self, out: bool, center_x: i32, center_y: i32) {
        self.zoom(out, (center_x as f32 * self.pixel_ratio).round() as i32, (center_y as f32 * self.pixel_ratio).round() as i32);
    }
    pub fn zoom_centered(&mut self, out: bool) {
        self.zoom(out, self.canvas_width >> 1, self.canvas_height >> 1);
    }
    pub fn zoom_to(&mut self, level: f32) {
        self.cell_width = level;
    }

    pub fn move_offset(&mut self, x: i32, y: i32) {
        self.canvas_offset_x += (x as f32 * self.pixel_ratio).round() as i32;
        self.canvas_offset_y += (y as f32 * self.pixel_ratio).round() as i32;
    }

    pub fn set_size(&mut self, width: i32, height: i32, factor: f32) {
        self.canvas_width = (width as f32 * factor).round() as i32;
        self.canvas_height = (height as f32 * factor).round() as i32;
        self.pixel_ratio = factor;
    }

    pub fn center_view(&mut self, offset_x: i32, offset_y: i32) {
        self.canvas_offset_x = self.canvas_width >> 1;
        self.canvas_offset_y = self.canvas_height >> 1;

        self.move_offset(-offset_x, -offset_y);
    }

    pub fn pixel_to_cell(&self, x: i32, y: i32) -> Vec<i32> {
        vec![
            ((x as f32 * self.pixel_ratio - self.canvas_offset_x as f32 + self.border_width / 2.0) / self.cell_width).round() as i32,
            ((y as f32 * self.pixel_ratio - self.canvas_offset_y as f32 + self.border_width / 2.0) / self.cell_width).round() as i32,
        ]
    }
    pub fn cell_to_pixel(&self, x: i32, y: i32) -> Vec<i32> {
        vec![
            ((x as f32 * self.cell_width + self.canvas_offset_x as f32 - self.border_width / 2.0) / self.pixel_ratio).round() as i32,
            ((y as f32 * self.cell_width + self.canvas_offset_y as f32 - self.border_width / 2.0) / self.pixel_ratio).round() as i32,
        ]
    }

    pub fn get_cell_width(&self) -> f32 {
        return self.cell_width
    }

    pub fn draw_cell(&mut self, x: i32, y: i32) {
        let width = self.cell_width - (self.cell_width * self.border_width);
        let pixels = self.cell_to_pixel(x, y);

        let (pixel_x, pixel_y) = (pixels[0], pixels[1]);
        self.added_cells.push((pixel_x, pixel_y, width));
    }

    fn draw_square(&mut self, mut x: i32, mut y: i32, size: f32, color: u32) {
        let mut width = size.round() as i32 - self.border_pixels;
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
                self.image_data_pixels[pointer as usize] = color;
                pointer += 1;
            }
            pointer += row_width;
        }
    }

    fn draw_node(&mut self, node: Option<Arc<Node>>, mut size: f32, left: f32, top: f32) {
        if node.population() == 0 { return };

        if left + size + (self.canvas_offset_x as f32) < 0.0 ||
            top + size + (self.canvas_offset_y as f32) < 0.0 ||
            left + (self.canvas_offset_x as f32) >= self.canvas_width as f32 ||
            top + (self.canvas_offset_y as f32) >= self.canvas_height as f32 { return };

        if size <= 1.0 {
            if node.population() > 0 {
                self.draw_square(left.round() as i32 + self.canvas_offset_x | 0, top.round() as i32 + self.canvas_offset_y | 0, 1.0, self.cell_color);
            }
        }
        else if node.level() == 0 {
            if node.population() > 0 {
                self.draw_square(left.round() as i32 + self.canvas_offset_x, top.round() as i32 + self.canvas_offset_y, self.cell_width, self.cell_color);
            }
        }
        else {
            size /= 2.0;

            self.draw_node(node.a(), size, left, top);
            self.draw_node(node.b(), size, left + size, top);
            self.draw_node(node.c(), size, left, top + size);
            self.draw_node(node.d(), size, left + size, top + size);
        }
    }

    pub fn get_image_data(&mut self, node: &Node) -> *const u8 {
        self.image_data_pixels = vec![self.background_color; (self.canvas_width * self.canvas_height) as usize];
        self.border_pixels = (self.border_width * self.cell_width as f32).floor() as i32 | 0;
        
        let size = 2.0_f32.powf(node.level() as f32 - 1.0) * self.cell_width;
        self.draw_node(Some(Arc::new(node.clone())), size * 2.0, -size, -size);
        
        for (x, y, width) in self.added_cells.drain(..).collect::<Vec<_>>() {
            self.draw_square(x, y, width, self.added_cell_color);
        }
        
        self.image_data_bytes = self.image_data_pixels.iter().flat_map(|val| val.to_be_bytes()).collect();

        return self.image_data_bytes.as_ptr()
    }
}