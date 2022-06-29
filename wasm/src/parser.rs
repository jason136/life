use wasm_bindgen::prelude::*;
use crate::{
    Life, 
    Node, 
    OptionExt,
};

fn to_rle(pts: Vec<(i32, i32)>) -> (String, (i32, i32)) {
    let max_x = pts.iter().map(|p| p.0).max().unwrap();
    let min_x = pts.iter().map(|p| p.0).min().unwrap();
    let max_y = pts.iter().map(|p| p.1).max().unwrap();
    let min_y = pts.iter().map(|p| p.1).min().unwrap();

    let mut line = 0;
    let mut x = 0;
    let mut stars = 0;
    let mut out: Vec<String> = Vec::new();

    let mut flush_stars = || {
        if stars == 1 {
            out.push("o".to_string());
        }
        if stars > 1 {
            out.push(format!("{}o", stars));
        }
    };

    for pt in pts.iter() {
        let pt = &(pt.0 - min_x, pt.1 - min_y);

        if pt.1 != line {
            flush_stars();

            let reps = pt.1 - line;
            if reps != 1 {
                out.push(format!("{}$", reps));
            }
            else {
                out.push("$".to_string());
            }

            line = pt.1;
            stars = 0; // line ~100 
            x = 0;
        }
    }

    return ("asd".to_string(), (23, 23))
}

#[wasm_bindgen]
impl Life {
    pub fn parse_rle(rle: String) -> Vec<i32> {
        let lines = rle.split("\n");

        let mut positions: Vec<i32> = Vec::new();

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut complete = false;
        for mut line in lines {
            line = line.trim();
            if line.len() == 0 {
                continue;
            }
            else if complete {
                // comments
            }
            else if line.starts_with("#") {
                // comments
                continue;
            }
            else if line.starts_with("x") {
                continue;
            }
            else {
                let mut count: i32 = 0;

                for char in line.chars() {
                    if char.is_digit(10) {
                        count *= 10;
                        count += char as i32; 
                    }
                    
                    match char.to_lowercase().next().unwrap() {
                        'b' => {
                            if count != 0 {
                                x += count;
                            }
                            else {
                                x += 1;
                            }
                            count = 0;
                        }
                        'o' => {
                            if count != 0 {
                                for _ in 0..count {
                                    positions.push(x);
                                    positions.push(y);
                                    x += 1;
                                }
                            }
                            else {
                                positions.push(x);
                                positions.push(y);
                                x += 1;
                            }
                            count = 0;
                        }
                        '$' => {
                            if count != 0 {
                                y += count;
                            }
                            else {
                                y += 1;
                            }
                            x = 0;
                            count = 0;
                        }
                        '!' => {
                            complete = true;
                            break;
                        }
                        _ => {
                            panic!("Unknown character: {}", char);
                        }
                    }
                }
            }
        }

        return positions;
    }

    pub fn convert_rle(pts: Vec<i32>) -> String {
        return "bla".to_string()
    }
}