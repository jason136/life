use wasm_bindgen::prelude::*;
use crate::{
    Node, 
    OptionExt,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Renderer {
    canvas_width: i32,
    canvas_height: i32,
    offset_x: i32,
    offset_y: i32,
    border_width: i32,
    cell_width: i32,
    zoom: f32,
    image_data_u32: Vec<u32>,
    image_data_u8: Vec<u8>,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            canvas_width: 0,
            canvas_height: 0,
            offset_x: 0,
            offset_y: 0,
            border_width: 1,
            cell_width: 5,
            zoom: 1.0,
            image_data_u32: vec![],
            image_data_u8: vec![],
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.canvas_width = width;
        self.canvas_height = height;
    }
    pub fn set_center(&mut self, x: i32, y: i32) {
        self.offset_x = x;
        self.offset_y = y;
    }
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    fn draw_square(&mut self, mut x: i32, mut y: i32, size: i32) {
        let mut height = size - self.border_width;
        let mut width = height;

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
                self.image_data_u32[pointer as usize] = 0xFFFFFFFF;
                pointer += 1;
            }
            pointer += row_width;
        }
    }

    fn draw_node(&mut self, node: Option<Arc<Node>>, mut size: i32, left: i32, top: i32) {
        if left + size + self.offset_x < 0 ||
            top + size + self.offset_y < 0 ||
            left + self.offset_x > self.canvas_width ||
            top + self.offset_y > self.canvas_height { return };
        

        if size <= 1 {
            if node.population() > 0 { self.draw_square(left + self.offset_x | 0, top + self.offset_y | 0, 1); }
        }
        else if node.level() == 0 {
            if node.population() > 0 { self.draw_square(left + self.offset_x, top + self.offset_y, self.cell_width); }
        }
        else {
            size /= 2;

            self.draw_node(node.a(), size, left, top);
            self.draw_node(node.b(), size, left + size, top);
            self.draw_node(node.c(), size, left, top + size);
            self.draw_node(node.d(), size, left + size, top + size);
        }
    }

    pub fn update_image_data(&mut self, node: Node) {
        self.image_data_u32 = vec![0xFFFFFFFF; (self.canvas_width * self.canvas_height) as usize];
        let size = 2_i32.pow(node.level() as u32 - 1) * self.cell_width;
        self.draw_node(Some(Arc::new(node)), size, -size, -size);

        self.image_data_u8 = self.image_data_u32.iter().flat_map(|val| val.to_be_bytes()).collect();
    }
    
    pub fn image_data_ptr(&self) -> *const u8 {
        self.image_data_u8.as_ptr()
    }
}