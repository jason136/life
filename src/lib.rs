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
    nw: Option<Box<Node>>,
    ne: Option<Box<Node>>,
    sw: Option<Box<Node>>,
    se: Option<Box<Node>>,
    population: u32,
    level: u32,
    hash: u64,
}

static ON: Node = Node{ nw: None, ne: None, sw: None, se: None, population: 1, level: 0, hash: 1 };
static OFF: Node = Node{ nw: None, ne: None, sw: None, se: None, population: 0, level: 0, hash: 0 };


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

    pub fn join(a: &Option<Box<Node>>, b: &Option<Box<Node>>, c: &Option<Box<Node>>, d: &Option<Box<Node>>) -> Node {
        let n_level = a.unwrap_ref().level() + 1;
        let n_population: u32 = a.unwrap_ref().population() + b.unwrap_ref().population() + c.unwrap_ref().population() + d.unwrap_ref().population();
        let n_hash: u64 = (a.unwrap_ref().hash() * 5131830419411
            + b.unwrap_ref().hash() * 3758991985019
            + c.unwrap_ref().hash() * 8973110871315
            + d.unwrap_ref().hash() * 4318490180473
            + u64::from(a.unwrap_ref().level())
        ) & ((1 << 63) - 1);
        
        Node {
            nw: a.clone(),
            ne: b.clone(),
            sw: c.clone(),
            se: d.clone(),
            population: n_population,
            level: n_level,
            hash: n_hash,
        }
    }

    pub fn get_zero(k: u32) -> Node {
        if k == 0 {
            OFF.clone()
        }
        else {
            Node::join (
                &Some(Box::new(Node::get_zero(k - 1))),
                &Some(Box::new(Node::get_zero(k - 1))),
                &Some(Box::new(Node::get_zero(k - 1))),
                &Some(Box::new(Node::get_zero(k - 1))),
            )
        }
    }

    pub fn center(mid: &Option<Box<Node>>) -> Node {
        let m = mid.unwrap_ref();
        let zero = Some(Box::new(Node::get_zero(mid.unwrap_ref().level() - 1)));
        Node::join (
            &Some(Box::new(Node::join(&zero, &zero, &zero, &m.clone().nw))),
            &Some(Box::new(Node::join(&zero, &zero, &m.clone().ne, &zero))),
            &Some(Box::new(Node::join(&zero, &m.clone().sw, &zero, &zero))), 
            &Some(Box::new(Node::join(&m.clone().se, &zero, &zero, &zero))),
        )
    }

    pub fn life(a: &Option<Box<Node>>, b: &Option<Box<Node>>, c: &Option<Box<Node>>, d: &Option<Box<Node>>, e: &Option<Box<Node>>, 
                f: &Option<Box<Node>>, g: &Option<Box<Node>>, h: &Option<Box<Node>>, i: &Option<Box<Node>>) -> Node{
        let mut outer = 0;
        for &n in [&a, &b, &c, &d, &e, &f, &g, &h, &i].iter() {
            if n.unwrap_ref().population() > 0 {
                outer += 1;
            }
        }
        match outer {
            3 => ON.clone(),
            2 => {
                if e.unwrap_ref().population() > 0 && outer == 2 {
                    ON.clone()
                }
                else {
                    OFF.clone()
                }
            }
            _ => OFF.clone()
        }
    }
    pub fn life_4x4(m: &Option<Box<Node>>) -> Node {
        let a = &m.unwrap_ref().nw.unwrap_ref();
        let b = &m.unwrap_ref().ne.unwrap_ref();
        let c = &m.unwrap_ref().sw.unwrap_ref();
        let d = &m.unwrap_ref().se.unwrap_ref();

        let ad = Node::life(&a.nw, &a.ne, &b.nw, &a.sw, &a.se, &b.sw, &c.nw, &c.ne, &d.nw);
        let bc = Node::life(&a.ne, &b.nw, &b.ne, &a.se, &b.sw, &b.se, &c.ne, &d.nw, &d.ne);
        let cb = Node::life(&a.sw, &a.se, &b.sw, &c.nw, &c.ne, &d.nw, &c.sw, &c.se, &d.sw);
        let da = Node::life(&a.se, &b.sw, &b.se, &c.ne, &d.nw, &d.ne, &c.se, &d.sw, &d.se);

        Node::join(
            &Some(Box::new(ad)),
            &Some(Box::new(bc)),
            &Some(Box::new(cb)),
            &Some(Box::new(da)),
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
    
//         let nw = self.get_index(north, west);
//         count += self.cells[nw] as u8;
    
//         let n = self.get_index(north, column);
//         count += self.cells[n] as u8;
    
//         let ne = self.get_index(north, east);
//         count += self.cells[ne] as u8;
    
//         let w = self.get_index(row, west);
//         count += self.cells[w] as u8;
    
//         let e = self.get_index(row, east);
//         count += self.cells[e] as u8;
    
//         let sw = self.get_index(south, west);
//         count += self.cells[sw] as u8;
    
//         let s = self.get_index(south, column);
//         count += self.cells[s] as u8;
    
//         let se = self.get_index(south, east);
//         count += self.cells[se] as u8;
    
//         count
//     }    
// }