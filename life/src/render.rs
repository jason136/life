use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use crate::{Node, OptionExt, NODE};
use std::sync::{Arc, Mutex};

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

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

static RENDERER: Lazy<Mutex<Renderer>> = Lazy::new(|| { Mutex::new(Renderer {
    background_color: 0x000000FF,
    cell_color: 0xFFFFFFFF,
    added_cell_color: 0xFFFFFFFF,

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
}) });

#[wasm_bindgen]
impl Renderer {
    pub fn set_background_color(color: u32) {
        RENDERER.lock().unwrap().background_color = color;
    }
    pub fn set_cell_color(color: u32) {
        RENDERER.lock().unwrap().cell_color = color;
    }

    fn zoom(renderer: &mut Renderer, out: bool, center_x: i32, center_y: i32) {
        if out {
            renderer.canvas_offset_x -= (renderer.canvas_offset_x - center_x) / 2;
            renderer.canvas_offset_y -= (renderer.canvas_offset_y - center_y) / 2;

            renderer.cell_width /= 2.0;
        }
        else {
            renderer.canvas_offset_x += renderer.canvas_offset_x - center_x;
            renderer.canvas_offset_y += renderer.canvas_offset_y - center_y;

            renderer.cell_width *= 2.0;
        }
    }

    pub fn zoom_at(out: bool, center_x: i32, center_y: i32) {
        let mut renderer = RENDERER.lock().unwrap();
        let pixel_ratio = renderer.pixel_ratio;
        Self::zoom(&mut *renderer, out, (center_x as f32 * pixel_ratio).round() as i32, (center_y as f32 * pixel_ratio).round() as i32);
    }
    pub fn zoom_centered(out: bool) {
        let mut renderer = RENDERER.lock().unwrap();
        let (width, height) = (renderer.canvas_width, renderer.canvas_height);
        Self::zoom(&mut *renderer, out, width >> 1, height >> 1);
    }
    pub fn zoom_to(level: f32) {
        RENDERER.lock().unwrap().cell_width = level;
    }

    pub fn move_offset(x: i32, y: i32) {
        let mut renderer = RENDERER.lock().unwrap();
        renderer.canvas_offset_x += (x as f32 * renderer.pixel_ratio).round() as i32;
        renderer.canvas_offset_y += (y as f32 * renderer.pixel_ratio).round() as i32;
    }

    pub fn set_size(width: i32, height: i32, factor: f32) {
        let mut renderer = RENDERER.lock().unwrap();
        renderer.canvas_width = (width as f32 * factor).round() as i32;
        renderer.canvas_height = (height as f32 * factor).round() as i32;
        renderer.pixel_ratio = factor;
    }

    pub fn center_view(offset_x: i32, offset_y: i32) {
        let mut renderer = RENDERER.lock().unwrap();
        renderer.canvas_offset_x = renderer.canvas_width >> 1;
        renderer.canvas_offset_y = renderer.canvas_height >> 1;

        renderer.canvas_offset_x += (-offset_x as f32 * renderer.pixel_ratio).round() as i32;
        renderer.canvas_offset_y += (-offset_y as f32 * renderer.pixel_ratio).round() as i32;
    }

    fn pixel_to_cell(renderer: &Renderer, x: i32, y: i32) -> Vec<i32> {
        vec![
            ((x as f32 * renderer.pixel_ratio - renderer.canvas_offset_x as f32 + renderer.border_width / 2.0) / renderer.cell_width).round() as i32,
            ((y as f32 * renderer.pixel_ratio - renderer.canvas_offset_y as f32 + renderer.border_width / 2.0) / renderer.cell_width).round() as i32,
        ]
    }
    fn cell_to_pixel(renderer: &Renderer, x: i32, y: i32) -> Vec<i32> {
        vec![
            ((x as f32 * renderer.cell_width + renderer.canvas_offset_x as f32 - renderer.border_width / 2.0) / renderer.pixel_ratio).round() as i32,
            ((y as f32 * renderer.cell_width + renderer.canvas_offset_y as f32 - renderer.border_width / 2.0) / renderer.pixel_ratio).round() as i32,
        ]
    }

    pub fn get_cell_width() -> f32 {
        RENDERER.lock().unwrap().cell_width
    }

    pub fn draw_cell(x: i32, y: i32) {
        let mut renderer = RENDERER.lock().unwrap();
        let width = renderer.cell_width;
        let cells = Self::pixel_to_cell(&*renderer, x, y);
        let pixels = Self::cell_to_pixel(&*renderer, cells[0], cells[1]);

        renderer.added_cells.push((pixels[0], pixels[1], width));
    }

    fn draw_square(renderer: &mut Renderer, mut x: i32, mut y: i32, size: f32, color: u32) {
        let mut width = size.round() as i32 - renderer.border_pixels;
        let mut height = width;

        if x < 0 {
            width += x;
            x = 0;
        }
        if x + width > renderer.canvas_width {
            width = renderer.canvas_width - x;
        }

        if y < 0 {
            height += y;
            y = 0;
        }
        if y + height > renderer.canvas_height {
            height = renderer.canvas_height - y;
        }
    
        if width <= 0 || height <= 0 {
            return;
        }

        let mut pointer = x + y * renderer.canvas_width;
        let row_width = renderer.canvas_width - width;

        for _ in 0..height {
            for _ in 0..width {
                renderer.image_data_pixels[pointer as usize] = color;
                pointer += 1;
            }
            pointer += row_width;
        }
    }

    fn draw_node(renderer: &mut Renderer, node: Option<Arc<Node>>, mut size: f32, left: f32, top: f32) {
        if node.population() == 0 { return };

        if left + size + (renderer.canvas_offset_x as f32) < 0.0 ||
            top + size + (renderer.canvas_offset_y as f32) < 0.0 ||
            left + (renderer.canvas_offset_x as f32) >= renderer.canvas_width as f32 ||
            top + (renderer.canvas_offset_y as f32) >= renderer.canvas_height as f32 { return };

        if size <= 1.0 {
            if node.population() > 0 {
                Self::draw_square(renderer, left.round() as i32 + renderer.canvas_offset_x | 0, top.round() as i32 + renderer.canvas_offset_y | 0, 1.0, renderer.cell_color);
            }
        }
        else if node.level() == 0 {
            if node.population() > 0 {
                Self::draw_square(renderer, left.round() as i32 + renderer.canvas_offset_x, top.round() as i32 + renderer.canvas_offset_y, renderer.cell_width, renderer.cell_color);
            }
        }
        else {
            size /= 2.0;

            Self::draw_node(renderer, node.a(), size, left, top);
            Self::draw_node(renderer, node.b(), size, left + size, top);
            Self::draw_node(renderer, node.c(), size, left, top + size);
            Self::draw_node(renderer, node.d(), size, left + size, top + size);
        }
    }

    pub fn get_image_data() -> *const u8 {
        let mut renderer = RENDERER.lock().unwrap();
        let node = NODE.lock().unwrap();

        renderer.image_data_pixels = vec![renderer.background_color; (renderer.canvas_width * renderer.canvas_height) as usize];
        renderer.border_pixels = (renderer.border_width * renderer.cell_width as f32).floor() as i32 | 0;
        
        let size = 2.0_f32.powf(node.level() as f32 - 1.0) * renderer.cell_width;
        Self::draw_node(&mut *renderer, node.clone(), size * 2.0, -size, -size);
        
        for (x, y, width) in renderer.added_cells.drain(..).collect::<Vec<_>>() {
            let new_cell_color = renderer.added_cell_color;
            Self::draw_square(&mut *renderer, x, y, width, new_cell_color);
        }
        
        renderer.image_data_bytes = renderer.image_data_pixels.iter().flat_map(|val| val.to_be_bytes()).collect();

        return renderer.image_data_bytes.as_ptr()
    }
}