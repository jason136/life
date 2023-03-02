use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::sync::{ Mutex, Arc };

mod render;
mod parser;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

type NodePtr = Option<Arc<Node>>;

static JOINCACHE: Lazy<Mutex<HashMap<u64, NodePtr>>> = Lazy::new(|| { Mutex::new(HashMap::new()) });
static ZEROCACHE: Lazy<Mutex<HashMap<u8, NodePtr>>> = Lazy::new(|| { Mutex::new(HashMap::new()) });
static SUCCESSORCACHE: Lazy<Mutex<HashMap<(u64, Option<u8>), NodePtr>>> = Lazy::new(|| { Mutex::new(HashMap::new()) });

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Node {
    a: NodePtr,
    b: NodePtr,
    c: NodePtr,
    d: NodePtr,
    population: u32,
    level: u8,
    hash: u64,
}

pub struct Life;

static ON: Node = Node{ a: None, b: None, c: None, d: None, population: 1, level: 0, hash: 1 };
static OFF: Node = Node{ a: None, b: None, c: None, d: None, population: 0, level: 0, hash: 0 };

trait OptionExt {
    fn hash(&self) -> u64;
    fn population(&self) -> u32;
    fn level(&self) -> u8;
    fn a(&self) -> NodePtr;
    fn b(&self) -> NodePtr;
    fn c(&self) -> NodePtr;
    fn d(&self) -> NodePtr;
}

impl OptionExt for NodePtr {
    fn hash(&self) -> u64 { self.as_ref().unwrap().hash }
    fn population(&self) -> u32 { self.as_ref().unwrap().population }
    fn level(&self) -> u8 { self.as_ref().unwrap().level }
    fn a(&self) -> NodePtr { self.as_ref().unwrap().a.clone() }
    fn b(&self) -> NodePtr { self.as_ref().unwrap().b.clone() }
    fn c(&self) -> NodePtr { self.as_ref().unwrap().c.clone() }
    fn d(&self) -> NodePtr { self.as_ref().unwrap().d.clone() }
}

// replace mutex lock with if let

// pending deletion
// #[wasm_bindgen]
// impl Node {
//     pub fn hash(&self) -> u64 { self.hash }
//     pub fn population(&self) -> u32 { self.population }
//     pub fn level(&self) -> u8 { self.level }
// }

fn join(a: NodePtr, b: NodePtr, c: NodePtr, d: NodePtr) -> NodePtr {
    let n_hash: u64 = (
        u64::from(a.level()) + 2 +
        &a.hash() * 2223243435546756677 +
        &b.hash() * 1241111124211111421 +
        &c.hash() * 7532753275327532753 +
        &d.hash() * 9876503214123056789
    ) & ((1 << 63) - 1);

    // if a.level() > 3 && JOINCACHE.lock().unwrap().contains_key(&n_hash) {
    //     return JOINCACHE.lock().unwrap().get(&n_hash).unwrap().clone()
    // }

    let n_level = &a.level() + 1;
    let n_population: u32 = &a.population() + &b.population() + &c.population() + &d.population();

    let n = Some(Arc::new(Node { 
        a, b, c, d,
        population: n_population,
        level: n_level,
        hash: n_hash,
    }));

    // JOINCACHE.lock().unwrap().insert(n_hash, n.clone());
    return n
}

fn get_zero(k: u8) -> NodePtr {
    if ZEROCACHE.lock().unwrap().contains_key(&k) {
        return ZEROCACHE.lock().unwrap().get(&k).unwrap().clone();
    }

    let n: NodePtr;
    if k == 0 {
        n = Some(Arc::new(OFF.clone()))
    }
    else {
        n = join (
            get_zero(k - 1),
            get_zero(k - 1),
            get_zero(k - 1),
            get_zero(k - 1),
        )
    }
    ZEROCACHE.lock().unwrap().insert(k, n.clone());
    return n;
}

fn life(a: NodePtr, b: NodePtr, c: NodePtr, d: NodePtr, e: NodePtr, 
        f: NodePtr, g: NodePtr, h: NodePtr, i: NodePtr) -> NodePtr {
    let mut outer = 0;
    for n in [a, b, c, d, f, g, h, i].iter() {
        outer += n.population();
    }

    if outer == 2 && e.population() > 0 {
        return Some(Arc::new(ON.clone()));
    }
    else if outer == 3 {
        return Some(Arc::new(ON.clone()));
    }
    else {
        Some(Arc::new(OFF.clone()))
    }
}
fn life_4x4(m: NodePtr) -> NodePtr {
    let na = life(m.a().a(), m.a().b(), m.b().a(), m.a().c(), m.a().d(), m.b().c(), m.c().a(), m.c().b(), m.d().a());
    let nb = life(m.a().b(), m.b().a(), m.b().b(), m.a().d(), m.b().c(), m.b().d(), m.c().b(), m.d().a(), m.d().b());
    let nc = life(m.a().c(), m.a().d(), m.b().c(), m.c().a(), m.c().b(), m.d().a(), m.c().c(), m.c().d(), m.d().c());
    let nd = life(m.a().d(), m.b().c(), m.b().d(), m.c().b(), m.d().a(), m.d().b(), m.c().d(), m.d().c(), m.d().d());

    join(na, nb, nc, nd)
}

use std::sync::atomic::{AtomicUsize, Ordering};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

fn successor(m: NodePtr, mut j: Option<u8>) -> NodePtr {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);

    if m.population() == 0 {
        return m.a();
    }

    if m.level() > 3 && SUCCESSORCACHE.lock().unwrap().contains_key(&(m.hash(), j)) {
        return SUCCESSORCACHE.lock().unwrap().get(&(m.hash(), j)).unwrap().clone();
    }

    let s: NodePtr;
    
    if m.level() == 2 {
        s = life_4x4(m.clone());
    }
    else {
        j = if j.is_none() {
            Some(m.level() - 2)
        } else {
            Some(std::cmp::min(j.unwrap(), m.level() - 2))
        };
    
        let c1 = successor(join(m.a().a(), m.a().b(), m.a().c(), m.a().d()), j);
        let c2 = successor(join(m.a().b(), m.b().a(), m.a().d(), m.b().c()), j);
        let c3 = successor(join(m.b().a(), m.b().b(), m.b().c(), m.b().d()), j);
        let c4 = successor(join(m.a().c(), m.a().d(), m.c().a(), m.c().b()), j);
        let c5 = successor(join(m.a().d(), m.b().c(), m.c().b(), m.d().a()), j);
        let c6 = successor(join(m.b().c(), m.b().d(), m.d().a(), m.d().b()), j);
        let c7 = successor(join(m.c().a(), m.c().b(), m.c().c(), m.c().d()), j);
        let c8 = successor(join(m.c().b(), m.d().a(), m.c().d(), m.d().c()), j);
        let c9 = successor(join(m.d().a(), m.d().b(), m.d().c(), m.d().d()), j);
    
        s = if j.unwrap() < m.level() - 2 { 
            join(
                join(c1.d(), c2.c(), c4.b(), c5.a()),
                join(c2.d(), c3.c(), c5.b(), c6.a()),
                join(c4.d(), c5.c(), c7.b(), c8.a()),
                join(c5.d(), c6.c(), c8.b(), c9.a()),
            ) } else { 
            join(
                successor(join(c1.clone(), c2.clone(), c4.clone(), c5.clone()), j),
                successor(join(c2.clone(), c3.clone(), c5.clone(), c6.clone()), j),
                successor(join(c4.clone(), c5.clone(), c7.clone(), c8.clone()), j),
                successor(join(c5.clone(), c6.clone(), c8.clone(), c9.clone()), j),
            )
        };
    }
    SUCCESSORCACHE.lock().unwrap().insert((m.hash(), j), s.clone());
    return s;
}

fn is_padded(node: NodePtr) -> bool {
    return 
        node.a().population() == node.a().d().d().population() &&
        node.b().population() == node.b().c().c().population() &&
        node.c().population() == node.c().b().b().population() &&
        node.d().population() == node.d().a().a().population()
}

fn inner(node: NodePtr) -> NodePtr {
    return join(
        node.a().d(),
        node.b().c(),
        node.c().b(),
        node.d().a()
    )
}

fn center(m: NodePtr) -> NodePtr {
    let z = get_zero(m.a().level());
    return join(
        join(z.clone(), z.clone(), z.clone(), m.a()), 
        join(z.clone(), z.clone(), m.b(), z.clone()), 
        join(z.clone(), m.c(), z.clone(), z.clone()), 
        join(m.d(), z.clone(), z.clone(), z),
    );
}

fn crop(node: NodePtr) -> NodePtr {
    if node.level() <= 3 || !is_padded(node.clone()) {
        return node
    }
    else {
        return crop(inner(node))
    }
}
fn pad(node: NodePtr) -> NodePtr {
    if node.level() <= 3 || !is_padded(node.clone()) {
        return pad(center(node))
    }
    else {
        return node
    }
}

fn expand_recurse(node: NodePtr, x: i32, y: i32) -> Vec<i32> {
    if node.population() == 0 {
        return Vec::new()
    }

    let size = 2_u32.pow(node.level() as u32);

    if node.level() == 0 {
        return vec![x, y]
    }
    else {
        let offset = (size >> 1) as i32;
        let mut output = Vec::new();
        output.append(&mut expand_recurse(node.a(), x, y));
        output.append(&mut expand_recurse(node.b(), x + offset, y));
        output.append(&mut expand_recurse(node.c(), x, y + offset));
        output.append(&mut expand_recurse(node.d(), x + offset, y + offset));
        return output
    }
}

fn set_cell_recurse(node: NodePtr, x: i32, y: i32, alive: bool) -> NodePtr {
    if node.as_ref().unwrap().level == 0 {
        if alive {
            return Some(Arc::new(ON.clone()))
        }
        else {
            return Some(Arc::new(OFF.clone()))
        }
    }
    
    let offset = (2_u32.pow(node.as_ref().unwrap().level as u32) >> 2) as i32;
    let (mut a, mut b, mut c, mut d) = (node.a(), node.b(), node.c(), node.d());

    if x >= 0 && y >= 0 {
        d = set_cell_recurse(node.d(), x - offset, y - offset, alive);
    }
    else if x >= 0 && y < 0 {
        c = set_cell_recurse(node.c(), x - offset, y + offset, alive);
    }
    else if x < 0 && y >= 0 {
        b = set_cell_recurse(node.b(), x + offset, y - offset, alive);
    }
    else {
        a = set_cell_recurse(node.a(), x + offset, y + offset, alive);
    }
    return join(a, b, c, d)
}

fn is_alive_recurse(node: NodePtr, x: i32, y: i32) -> bool {
    if node.level() == 0 {
        if node.population() == 1 {
            return true
        }
        else {
            return false
        }
    }
    else {
        let offset = (2_u32.pow(node.level() as u32) >> 2) as i32;

        if x >= 0 && y >= 0 {
            return is_alive_recurse(node.d(), x - offset, y - offset)
        }
        else if x >= 0 && y < 0 {
            return is_alive_recurse(node.c(), x - offset, y + offset)
        }
        else if x < 0 && y >= 0 {
            return is_alive_recurse(node.b(), x + offset, y - offset)
        }
        else {
            return is_alive_recurse(node.a(), x + offset, y + offset)
        }
    }
}

fn get_bounds_recurse(node: NodePtr, x: i32, y: i32, border: &str) -> Vec<(i32, i32, &str)> {
    if node.population() == 0 {
        return vec![(0, 0, border)]
    }
    if node.level() == 0 {
        return vec![(x, y, border)]
    }
    else {
        let offset = (2_u32.pow(node.level() as u32) >> 2) as i32;
        let mut output = Vec::new();

        if border == "left" {
            if node.a().population() > 0  || node.c().population() > 0{
                output.append(&mut get_bounds_recurse(node.a(), x - offset, y - offset, "left"));
                output.append(&mut get_bounds_recurse(node.c(), x - offset, y + offset, "left"));

            }
            else {
                output.append(&mut get_bounds_recurse(node.b(), x + offset, y - offset, "left"));
                output.append(&mut get_bounds_recurse(node.d(), x + offset, y + offset, "left"));
            }
        }
        else {
            if node.a().population() > 0 || node.b().population() > 0 {
                output.append(&mut get_bounds_recurse(node.a(), x - offset, y - offset, "top"));
                output.append(&mut get_bounds_recurse(node.b(), x + offset, y - offset, "top"));
            }
            else {
                output.append(&mut get_bounds_recurse(node.c(), x - offset, y + offset, "top"));
                output.append(&mut get_bounds_recurse(node.d(), x + offset, y + offset, "top"));
            }
        }
        return output
    }
}

#[wasm_bindgen]
impl Life {
    pub fn expand(node: &Node, x: i32, y: i32) -> Vec<i32> {
        let node_arc = Some(Arc::new(node.clone()));

        let mut output = expand_recurse(node_arc, x, y);
        let min_x = output.chunks(2).map(|c| c[0]).min().unwrap();
        let min_y = output.chunks(2).map(|c| c[1]).min().unwrap();
        let min = std::cmp::min(min_x, min_y);

        output = output.iter().map(|c| c - min).collect();
        return output
    }

    pub fn construct(pts: Vec<i32>) -> Node {
        if pts.len() == 0 || pts.len() % 2 == 1 {
            return (*get_zero(4).as_deref().unwrap()).clone()
        }

        let x_vals: Vec::<i32> = pts.chunks(2).map(|c| c[0]).collect();
        let y_vals: Vec::<i32> = pts.chunks(2).map(|c| c[1]).collect();
        let min_x = x_vals.iter().min().unwrap();
        let min_y = y_vals.iter().min().unwrap();

        let mut pattern: HashMap<(i32, i32), NodePtr> = std::collections::HashMap::new();
        for n in 0..x_vals.len() {
            pattern.insert(
                (x_vals[n] - min_x, y_vals[n] - min_y),
                Some(Arc::new(ON.clone()))
            );
        }

        let mut k = 0;
        let mut last_updated = (0, 0);
        while pattern.len() != 1 {
            let mut next_level = std::collections::HashMap::new();
            let z = get_zero(k);

            while pattern.len() > 0 {
                let (mut x, mut y) = pattern.iter().next().unwrap().0;
                x = x - (x & 1);
                y = y - (y & 1);

                let a = pattern.remove(&(x, y)).unwrap_or(z.clone());
                let b = pattern.remove(&(x + 1, y)).unwrap_or(z.clone());
                let c = pattern.remove(&(x, y + 1)).unwrap_or(z.clone());
                let d = pattern.remove(&(x + 1, y + 1)).unwrap_or(z.clone());

                last_updated = (x >> 1, y >> 1);
                next_level.insert((x >> 1, y >> 1), join(a, b, c, d));

            }
            pattern = next_level;
            k += 1;
        }
        return (*pad(pattern[&last_updated].clone()).unwrap()).clone()
    }

    pub fn advance(node: Node, mut n: u32) -> Node {
        if n == 0 {
            return node
        }

        let mut node_arc = Some(Arc::new(node));
        let mut bits = Vec::new();
        while n > 0 {
            bits.push(n & 1);
            n = n >> 1;
            node_arc = center(node_arc);
        }

        for (k, bit) in bits.iter().rev().enumerate() {
            let j: u8 = (bits.iter().len() - k - 1).try_into().unwrap();
            if bit != &0 {
                node_arc = successor(pad(node_arc), Some(j));
            }
        }

        log(format!("{:?}", CALL_COUNT.load(Ordering::SeqCst)).as_str());
        CALL_COUNT.store(0, Ordering::SeqCst);

        return (*crop(node_arc).unwrap()).clone()
    }

    pub fn is_alive(node: &mut Node, x: i32, y: i32) -> bool {
        let node_arc = Some(Arc::new(node.clone()));
        // best not to ask why x and y are swapped
        return is_alive_recurse(node_arc, y, x);
    }

    pub fn set_cell(node: Node, x: i32, y: i32, alive: bool) -> Node {
        let node_arc = Some(Arc::new(node.clone()));
        let new_node = set_cell_recurse(node_arc, y, x, alive);
        return (*new_node.unwrap()).clone()
    }

    pub fn get_bounds(node: &Node) -> Vec<i32> {
        let node_arc = Some(Arc::new(node.clone()));

        let left = get_bounds_recurse(node_arc.clone(), 0, 0, "left");
        let top = get_bounds_recurse(node_arc.clone(), 0, 0, "top");

        let min_x = left.iter().map(|x| x.0).min().unwrap() + 1;
        let min_y = top.iter().map(|y| y.1).min().unwrap() + 1;

        let pts = Life::expand(node, 0, 0);

        let x = pts.chunks(2).map(|c| c[0]).max().unwrap();
        let y = pts.chunks(2).map(|c| c[1]).max().unwrap();

        return vec![min_x, min_x + x, min_y, min_y + y]
    }

    // pub fn get_bounds(node: &Node) -> Vec<i32> {
    //     let pts = Life::expand(node, 0, 0);
    //     let pairs = pts.chunks(2);

    //     let min_x = &pairs.clone().map(|p| p[0]).min().unwrap();
    //     let max_x = &pairs.clone().map(|p| p[0]).max().unwrap();
    //     let min_y = &pairs.clone().map(|p| p[1]).min().unwrap();
    //     let max_y = &pairs.clone().map(|p| p[1]).max().unwrap();

    //     return vec![min_x.clone(), max_x.clone(), min_y.clone(), max_y.clone()]
    // }

    // needs revision, don't use for now.
    pub fn ffwd(node: &Node, n: u32) -> Node {
        let mut node = Some(Arc::new(node.clone()));
        for _ in 0..n {
            while node.level() < 3 || 
                node.a().population() != node.a().d().d().population() ||
                node.b().population() != node.b().c().c().population() ||
                node.c().population() != node.c().b().b().population() ||
                node.d().population() != node.d().a().a().population() {
                node = center(node);
            }
            node = successor(node, None);
        }
        return (*node.unwrap()).clone()
    }
}

// https://github.com/johnhw/hashlife/blob/master/hashlife.py
// https://rustwasm.github.io/docs/book/game-of-life/implementing.html