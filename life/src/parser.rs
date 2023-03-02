use wasm_bindgen::prelude::*;
use crate::Life;
use regex::Regex;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn to_rle(mut pts: Vec<(i32, i32)>) -> (String, (i32, i32)) {
    let max_x = pts.iter().map(|p| p.0).max().unwrap();
    let max_y = pts.iter().map(|p| p.1).max().unwrap();
    let min_x = pts.iter().map(|p| p.0).min().unwrap();
    let min_y = pts.iter().map(|p| p.1).min().unwrap();

    pts.sort_by(|a, b| (&a.1).cmp(&b.1));

    let mut line = 0;
    let mut x = 0;
    let mut stars = 0;
    let mut out: Vec<String> = Vec::new();

    fn flush_stars(stars: i32, mut out: Vec<String>) -> Vec<String> {
        if stars == 1 {
            out.push("o".to_string());
        }
        else if stars > 1 {
            out.push(format!("{}o", stars));
        }
        return out
    }

    for pt in pts.iter() {
        let pt = (pt.0 - min_x, pt.1 - min_y);

        if pt.1 != line {
            out = flush_stars(stars, out);

            let reps = pt.1 - line;
            if reps != 1 {
                out.push(format!("{}$", reps));
            }
            else {
                out.push("$".to_string());
            }

            line = pt.1;
            stars = 0;
            x = 0;
        }

        let mut cts = 0;
        while x != pt.0 {
            x += 1;
            cts += 1;
        }

        if cts != 0 {
            out = flush_stars(stars, out);

            if cts == 1 {
                out.push("b".to_string());
            }
            else {
                out.push(format!("{}b", cts));
            }
            stars = 0;
        }

        stars += 1;
        x += 1;
    }

    out = flush_stars(stars, out);
    out.push("!".to_string());
    return (out.join(""), (max_x - min_x, max_y - min_y))
}

#[wasm_bindgen]
impl Life {
    pub fn parse_rle(rle: String) -> Vec<i32> {
        let lines = rle.split("\n");
        let mut positions: Vec<i32> = Vec::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut complete = false;

        for line in lines {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            else if complete {
                // comments
                continue;
            }
            else if line.starts_with("#") {    
                if "cCoOnN".contains(line.chars().nth(1).unwrap()) {
                    // comments
                    continue;
                }
                else if "pP".contains(line.chars().nth(1).unwrap()) {
                    let coords = line[2..].to_string();
                    for p in coords.chars() {
                        if p.is_digit(10) {
                            if x == 0 {
                                x = p.to_digit(10).unwrap() as i32;
                            }
                            else {
                                y = p.to_digit(10).unwrap() as i32;
                            }
                        }
                    }
                }
            }
            else if line.starts_with("x") {
                // size is not needed
                continue;
            }
            else {
                let mut count: i32 = 0;

                for char in line.chars() {
                    if char.is_digit(10) {
                        count *= 10;
                        count += char.to_string().parse::<i32>().unwrap();
                        continue;
                    }

                    count = if count == 0 { 1 } else { count };
                    match char.to_lowercase().next().unwrap() {
                        'b' => {
                            x += count;
                        }
                        'o' => {
                            for _ in 0..count {
                                positions.push(x);
                                positions.push(y);
                                x += 1;
                            }
                        }
                        '$' => {
                            y += count;
                            x = 0;
                        }
                        '!' => {
                            complete = true;
                            break;
                        }
                        _ => {
                            panic!("Unknown character: {}", char);
                        }
                    }
                    count = 0;
                }
            }
        }

        return positions;
    }

    pub fn convert_rle(pts: Vec<i32>, comment_string: String) -> String {
        let (rle, (x, y)) = to_rle(pts.chunks(2).map(|c| (c[0], c[1])).collect());
        let mut output = Vec::new();

        let comments = comment_string.split("\n");
        for comment in comments {
            output.push(format!("#C {}", comment));
        }
        output.push(format!("x = {}, y = {}\n", x, y));

        let mut wrapped = "".to_string();
        let mut index = 0;
        for char in rle.chars() {
            if index <= 70 {
                wrapped.push_str(&char.to_string());
                index += 1;
            }
            else {
                wrapped.push_str("\n");
                index = 0;
            }
        }
        output.push(wrapped);
        
        return output.join("\n");
    }

    pub fn parse_life106(text: String) -> Vec<i32> {
        let lines = text.split("\n");
        let mut positions: Vec<i32> = Vec::new();

        let pattern_106 = r"\s*\-?[0-9]+\s+\-?[0-9]+\s*";
        let re = Regex::new(pattern_106).unwrap();
        for line in lines {
            let line = line.trim();
            
            if line.starts_with('#') {
                // comments
                continue;
            }
            else if re.is_match(line) {
                let parts: Vec<&str> = line.split("").collect();
                for i in 0..&parts.len() / 2 {
                    let x = parts[2 * i].parse::<i32>().unwrap();
                    let y = parts[2 * i + 1].parse::<i32>().unwrap();
                    positions.push(x);
                    positions.push(y);
                }
            }
        }

        return positions;
    }
}