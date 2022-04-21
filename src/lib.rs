mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0, 
    Alive = 1,
}

// #[wasm_bindgen]
// pub struct Universe {
//     width: u32,
//     height: u32,
//     cells: FixedBitSet,
// }

#[derive(Clone)]
#[wasm_bindgen]
pub struct Node {
    a: Option<Box<Node>>,
    b: Option<Box<Node>>,
    c: Option<Box<Node>>,
    d: Option<Box<Node>>,
    population: u32,
    level: u32,
    hash: u64,
}

static ON: Node = Node{ a: None, b: None, c: None, d: None, population: 1, level: 0, hash: 1 };
static OFF: Node = Node{ a: None, b: None, c: None, d: None, population: 0, level: 0, hash: 0 };


trait OptionExt {
    type Value;
    fn unwrap_ref(&self) -> &Self::Value;
    fn unwrap_mut(&mut self) -> &mut Self::Value;
}

impl <T> OptionExt for Option<T> {
    type Value = T;
    fn unwrap_ref(&self) -> &T { self.as_ref().unwrap() }
    fn unwrap_mut(&mut self) -> &mut T { self.as_mut().unwrap() }
}

#[wasm_bindgen]
impl Node {
    pub fn hash(&self) -> u64 {
        self.hash
    }
    pub fn population(&self) -> u32 {
        self.population
    }
    pub fn level(&self) -> u32 {
        self.level
    }
    pub fn a(&self) -> Node {
        *self.a.unwrap_ref().clone()
    }
    pub fn b(&self) -> Node {
        *self.b.unwrap_ref().clone()
    }
    pub fn c(&self) -> Node {
        *self.c.unwrap_ref().clone()
    }
    pub fn d(&self) -> Node {
        *self.d.unwrap_ref().clone()
    }
}

fn join(a: &Node, b: &Node, c: &Node, d: &Node) -> Node {
    let n_level = a.level() + 1;
    let n_population: u32 = a.population() + b.population() + c.population() + d.population();
    let n_hash: u64 = (a.hash() * 5131830419411
        + b.hash() * 3758991985019
        + c.hash() * 8973110871315
        + d.hash() * 4318490180473
        + u64::from(a.level())
    ) & ((1 << 63) - 1);
    
    Node {
        a: Some(Box::new(a.clone())),
        b: Some(Box::new(b.clone())),
        c: Some(Box::new(c.clone())),
        d: Some(Box::new(d.clone())),
        population: n_population,
        level: n_level,
        hash: n_hash,
    }
}

fn get_zero(k: u32) -> Node {
    if k == 0 {
        OFF.clone()
    }
    else {
        join (
            &get_zero(k - 1),
            &get_zero(k - 1),
            &get_zero(k - 1),
            &get_zero(k - 1),
        )
    }
}

fn center(mid: Option<Box<Node>>) -> Node {
    let m = mid.unwrap_ref();
    let zero = &get_zero(mid.unwrap_ref().level() - 1);
    join (
        &join(zero, zero, zero, m.a.unwrap_ref()),
        &join(zero, zero, m.b.unwrap_ref(), zero),
        &join(zero, m.c.unwrap_ref(), zero, zero),
        &join(m.d.unwrap_ref(), zero, zero, zero),
    )
}

fn life(a: &Node, b: &Node, c: &Node, d: &Node, e: &Node, 
            f: &Node, g: &Node, h: &Node, i: &Node) -> Node{
    let mut outer = 0;
    for &n in [&a, &b, &c, &d, &e, &f, &g, &h, &i].iter() {
        if n.population() > 0 {
            outer += 1;
        }
    }
    match outer {
        3 => ON.clone(),
        2 => {
            if e.population() > 0 && outer == 2 {
                ON.clone()
            }
            else {
                OFF.clone()
            }
        }
        _ => OFF.clone()
    }
}
fn life_4x4(m: &Node) -> Node {
    let a = &m.a.unwrap_ref();
    let b = &m.b.unwrap_ref();
    let c = &m.c.unwrap_ref();
    let d = &m.d.unwrap_ref();

    let ad = life(&a.a(), &a.b(), &b.a(), &a.c(), &a.d(), &b.c(), &c.a(), &c.b(), &d.a());
    let bc = life(&a.b(), &b.a(), &b.b(), &a.d(), &b.c(), &b.d(), &c.b(), &d.a(), &d.b());
    let cb = life(&a.c(), &a.d(), &b.c(), &c.a(), &c.b(), &d.a(), &c.c(), &c.d(), &d.c());
    let da = life(&a.d(), &b.c(), &b.d(), &c.b(),&d.a(), &d.b(), &c.d(), &d.c(), &d.d());

    join(
        &ad,
        &bc,
        &cb,
        &da,
    )
}

fn successor(m: &Node, j: Option<u32>) -> Node {
    let a = &m.a.unwrap_ref();
    let b = &m.b.unwrap_ref();
    let c = &m.c.unwrap_ref();
    let d = &m.d.unwrap_ref();

    if m.level() == 0 {
        m.a().clone()
    }
    else if m.hash() == 2 {
        life_4x4(m)
    }
    else {
        if j.is_none() {
            j = Some(m.hash() - 2);
        }
        else {
            
        }

        let c1 = successor(&join(&a.a(), &a.b(), &a.c(), &a.d()), j);
        let c2 = successor(&join(&a.b(), &b.a(), &a.d(), &b.c()), j);
        let c3 = successor(&join(&b.a(), &b.b(), &b.c(), &b.d()), j);
        let c4 = successor(&join(&a.c(), &a.d(), &c.a(), &c.b()), j);
        let c5 = successor(&join(&a.d(), &b.c(), &c.b(), &d.a()), j);
        let c6 = successor(&join(&b.c(), &b.d(), &d.a(), &d.b()), j);
        let c7 = successor(&join(&c.a(), &c.b(), &c.c(), &c.d()), j);
        let c8 = successor(&join(&c.b(), &d.a(), &c.d(), &d.c()), j);
        let c9 = successor(&join(&d.a(), &d.b(), &d.c(), &d.d()), j);

        join(
            &successor(&join(&c1.d(), &c2.c(), &c4.b(), &c5.a()), j),
            &successor(&join(&c2.d(), &c3.c(), &c5.b(), &c6.a()), j),
            &successor(&join(&c4.d(), &c5.c(), &c7.b(), &c8.a()), j),
            &successor(&join(&c5.d(), &c6.c(), &c8.b(), &c9.a()), j)
        )
    }
}

// #[wasm_bindgen]
// impl Universe {
//     pub fn tick(&mut self) {
//         let mut next = self.cells.clone();
//         for row in 0..self.width {
//             for col in 0..self.height {
//                 let idx = self.get_index(row, col);
//                 let cell = self.cells[idx];
//                 let live_neighbors = self.live_neighbor_count(row, col);

//                 next.set(idx, match (cell, live_neighbors) {
//                     (true, x) if x < 2 => false,
//                     (true, 2) | (true, 3) => true,
//                     (true, x) if x > 3 => false,
//                     (false, 3) => true,
//                     (otherwise, _) => otherwise,
//                 });
//             }
//         }
//         self.cells = next;
//     }

//     pub fn new(width: u32, height: u32) -> Universe {
//         utils::set_panic_hook();

//         let size = (width * height) as usize;
//         let mut cells = FixedBitSet::with_capacity(size);

//         for i in 0..size {
//             // cells.set(i, js_sys::Math::random() < 0.5);
//             cells.set(i, i % 2 == 0 || i % 7 == 0);
//         }

//         Universe {
//             width, 
//             height, 
//             cells,
//         }
//     }

//     pub fn width(&self) -> u32 {
//         self.width
//     }
//     pub fn height(&self) -> u32 {
//         self.height
//     }
//     pub fn set_width(&mut self, width: u32) {
//         self.width = width;
//         self.cells = FixedBitSet::with_capacity((width * self.height) as usize);
//     }
//     pub fn set_height(&mut self, height: u32) {
//         self.height = height;
//         self.cells = FixedBitSet::with_capacity((self.width * height) as usize);
//     }

//     pub fn cells(&self) -> *const u32 {
//         self.cells.as_slice().as_ptr()
//     }
//     pub fn toggle_cell(&mut self, row: u32, column: u32) {
//         let idx = self.get_index(row, column);
//         self.cells.toggle(idx);
//     }
// }

// // impl Universe {
//     fn get_index(&self, row: u32, column: u32) -> usize {
//         (row * self.width + column) as usize
//     }
//     pub fn get_cells(&self) -> &FixedBitSet {
//         &self.cells
//     }
//     pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
//         for (row, col) in cells.iter().cloned() {
//             let idx = self.get_index(row, col);
//             self.cells.set(idx, true);
//         }
//     }
    
//     fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
//         let mut count = 0;
    
//         let north = if row == 0 {
//             self.height - 1
//         } else {
//             row - 1
//         };
    
//         let south = if row == self.height - 1 {
//             0
//         } else {
//             row + 1
//         };
    
//         let west = if column == 0 {
//             self.width - 1
//         } else {
//             column - 1
//         };
    
//         let east = if column == self.width - 1 {
//             0
//         } else {
//             column + 1
//         };
    
//         let a = self.get_index(north, west);
//         count += self.cells[a] as u8;
    
//         let n = self.get_index(north, column);
//         count += self.cells[n] as u8;
    
//         let b = self.get_index(north, east);
//         count += self.cells[b] as u8;
    
//         let w = self.get_index(row, west);
//         count += self.cells[w] as u8;
    
//         let e = self.get_index(row, east);
//         count += self.cells[e] as u8;
    
//         let c = self.get_index(south, west);
//         count += self.cells[c] as u8;
    
//         let s = self.get_index(south, column);
//         count += self.cells[s] as u8;
    
//         let d = self.get_index(south, east);
//         count += self.cells[d] as u8;
    
//         count
//     }    
// }