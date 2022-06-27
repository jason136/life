use wasm_bindgen::prelude::*;

mod render;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

lazy_static! {
    static ref JOINCACHE: Mutex<HashMap<(u64, u64, u64, u64), Option<Arc<Node>>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
    static ref ZEROCACHE: Mutex<HashMap<u8, Option<Arc<Node>>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
    static ref SUCCESSORCACHE: Mutex<HashMap<(u64, Option<u8>), Option<Arc<Node>>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}
// note: memoize only for larger inputs

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Node {
    a: Option<Arc<Node>>,
    b: Option<Arc<Node>>,
    c: Option<Arc<Node>>,
    d: Option<Arc<Node>>,
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
    fn a(&self) -> Option<Arc<Node>>;
    fn b(&self) -> Option<Arc<Node>>;
    fn c(&self) -> Option<Arc<Node>>;
    fn d(&self) -> Option<Arc<Node>>;
}

impl OptionExt for Option<Arc<Node>> {
    fn hash(&self) -> u64 { self.as_ref().unwrap().hash }
    fn population(&self) -> u32 { self.as_ref().unwrap().population }
    fn level(&self) -> u8 { self.as_ref().unwrap().level }
    fn a(&self) -> Option<Arc<Node>> { self.as_ref().unwrap().as_ref().a.clone() }
    fn b(&self) -> Option<Arc<Node>> { self.as_ref().unwrap().as_ref().b.clone() }
    fn c(&self) -> Option<Arc<Node>> { self.as_ref().unwrap().as_ref().c.clone() }
    fn d(&self) -> Option<Arc<Node>> { self.as_ref().unwrap().as_ref().d.clone() }
}

#[wasm_bindgen]
impl Node {
    pub fn hash(&self) -> u64 {
        self.hash
    }
    pub fn population(&self) -> u32 {
        self.population
    }
    pub fn level(&self) -> u8 {
        self.level
    }
}

fn join(a: Option<Arc<Node>>, b: Option<Arc<Node>>, c: Option<Arc<Node>>, d: Option<Arc<Node>>) -> Option<Arc<Node>> {
    let hash_tuple = (a.hash(), b.hash(), c.hash(), d.hash());
    if a.level() > 3 && JOINCACHE.lock().unwrap().contains_key(&hash_tuple) {
        let n = JOINCACHE.lock().unwrap().get(&hash_tuple).unwrap().clone();
        return n;
    }

    let n_level = &a.level() + 1;
    let n_population: u32 = &a.population() + &b.population() + &c.population() + &d.population();
    let n_hash: u64 = (
        u64::from(a.level()) + 2 +
        &a.hash() * 5131830419411 +
        &b.hash() * 3758991985019 +
        &c.hash() * 8973110871315 +
        &d.hash() * 4318490180473
    ) & ((1 << 63) - 1);

    let n = Some(Arc::new(Node {
        a,
        b,
        c,
        d,
        population: n_population,
        level: n_level,
        hash: n_hash,
    }));

    JOINCACHE.lock().unwrap().insert(hash_tuple, n.clone());
    return n
}

fn get_zero(k: u8) -> Option<Arc<Node>> {
    if ZEROCACHE.lock().unwrap().contains_key(&k) {
        let n = ZEROCACHE.lock().unwrap().get(&k).unwrap().clone();
        return n;
    }

    let n: Option<Arc<Node>>;
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

fn center(m: Option<Arc<Node>>) -> Option<Arc<Node>> {
    let zero = get_zero(m.a().level());
    join (
        join(zero.clone(), zero.clone(), zero.clone(), m.a()),
        join(zero.clone(), zero.clone(), m.b(),zero.clone()),
        join(zero.clone(), m.c(), zero.clone(),zero.clone()),
        join(m.d(), zero.clone(), zero.clone(),zero),
    )
}

fn life(a: Option<Arc<Node>>, b: Option<Arc<Node>>, c: Option<Arc<Node>>, d: Option<Arc<Node>>, e: Option<Arc<Node>>, 
        f: Option<Arc<Node>>, g: Option<Arc<Node>>, h: Option<Arc<Node>>, i: Option<Arc<Node>>) -> Option<Arc<Node>> {
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
fn life_4x4(m: Option<Arc<Node>>) -> Option<Arc<Node>> {
    let na = life(m.a().a(), m.a().b(), m.b().a(), m.a().c(), m.a().d(), m.b().c(), m.c().a(), m.c().b(), m.d().a());
    let nb = life(m.a().b(), m.b().a(), m.b().b(), m.a().d(), m.b().c(), m.b().d(), m.c().b(), m.d().a(), m.d().b());
    let nc = life(m.a().c(), m.a().d(), m.b().c(), m.c().a(), m.c().b(), m.d().a(), m.c().c(), m.c().d(), m.d().c());
    let nd = life(m.a().d(), m.b().c(), m.b().d(), m.c().b(), m.d().a(), m.d().b(), m.c().d(), m.d().c(), m.d().d());

    join(
        na,
        nb,
        nc,
        nd,
    )
}

use std::sync::atomic::{AtomicUsize, Ordering};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

fn successor(m: Option<Arc<Node>>, mut j: Option<u8>) -> Option<Arc<Node>> {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);

    if m.population() == 0 {
        return m.a().clone()
    }

    // Successor cache causes incorrect behavior for now
    // if SUCCESSORCACHE.lock().unwrap().contains_key(&(m.hash(), j)) {
    //     return SUCCESSORCACHE.lock().unwrap().get(&(m.hash(), j)).unwrap().clone();
    // }

    let s: Option<Arc<Node>>;
    
    if m.level() == 2 {
        s = life_4x4(m.clone())
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
                join(c5.d(), c6.c(), c8.b(), c9.a())
            ) } else { 
            join(
                successor(join(c1, c2.clone(), c4.clone(), c5.clone()), j),
                successor(join(c2, c3, c5.clone(), c6.clone()), j),
                successor(join(c4, c5.clone(), c7, c8.clone()), j),
                successor(join(c5, c6, c8, c9), j),
            )
        };
    }

    SUCCESSORCACHE.lock().unwrap().insert((s.hash(), j), s.clone());
    return s;
}

// These can be impl of Node if need be.
fn is_padded(node: Option<Arc<Node>>) -> bool {
    return 
        node.a().population() == node.a().d().d().population() &&
        node.b().population() == node.b().c().c().population() &&
        node.c().population() == node.c().b().b().population() &&
        node.d().population() == node.d().a().a().population()
}

fn inner(node: Option<Arc<Node>>) -> Option<Arc<Node>> {
    return join(
        node.a().d(),
        node.b().c(),
        node.c().b(),
        node.d().a()
    )
}

fn crop(node: Option<Arc<Node>>) -> Option<Arc<Node>> {
    if node.level() <= 3 || !is_padded(node.clone()) {
        return node
    }
    else {
        return crop(inner(node))
    }
}
fn pad(node: Option<Arc<Node>>) -> Option<Arc<Node>> {
    if node.level() <= 3 || !is_padded(node.clone()) {
        return pad(center(node))
    }
    else {
        return node
    }
}


#[wasm_bindgen]
impl Life {
    pub fn expand(node: &Node, x: i32, y: i32) -> Vec<i32> {
        if node.population() == 0 {
            return Vec::new()
        }

        let size = 2_u32.pow(node.level() as u32);

        if node.level() == 0 {
            return vec![x, y]
        }
        else {
            let offset = (size >> 1) as i32;
            let mut new = Vec::new();
            new.append(&mut Life::expand(&node.a.as_ref().unwrap(), x, y));
            new.append(&mut Life::expand(&node.b.as_ref().unwrap(), x + offset, y));
            new.append(&mut Life::expand(&node.c.as_ref().unwrap(), x, y + offset));
            new.append(&mut Life::expand(&node.d.as_ref().unwrap(), x + offset, y + offset));
            return new
        }
    }

    pub fn construct(pts: Vec<i32>) -> Node {
        if pts.len() == 0 || pts.len() % 2 == 1 {
            return (*get_zero(4).as_deref().unwrap()).clone()
        }

        let mut x_vals = Vec::new();
        let mut y_vals = Vec::new();
        for n in 0..pts.len() {
            if n % 2 == 0 {
                x_vals.push(pts[n]);
            }
            else {
                y_vals.push(pts[n]);
            }
        }
        let x_min = x_vals.iter().min().unwrap();
        let y_min = y_vals.iter().min().unwrap();
        
        let mut pattern = std::collections::HashMap::new();
        for n in 0..x_vals.len() {
            pattern.insert(
                (x_vals[n] - x_min, y_vals[n] - y_min),
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

                let (a, b, c, d): (Option<Arc<Node>>, Option<Arc<Node>>, Option<Arc<Node>>, Option<Arc<Node>>);
                if let Some(n) = pattern.remove(&(x, y)) {
                    a = n;
                } else {
                    a = z.clone();
                }
                if let Some(n) = pattern.remove(&(x + 1, y)) {
                    b = n;
                } else {
                    b = z.clone();
                }
                if let Some(n) = pattern.remove(&(x, y + 1)) {
                    c = n;
                } else {
                    c = z.clone();
                }
                if let Some(n) = pattern.remove(&(x + 1, y + 1)) {
                    d = n;
                } else {
                    d = z.clone();
                }

                last_updated = (x >> 1, y >> 1);
                next_level.insert((x >> 1, y >> 1), join(a, b, c, d));

            }
            pattern = next_level;
            k += 1;
        }

        return (*pad(pattern[&last_updated].clone()).as_deref().unwrap()).clone()
    }

    // has potential to be better than above but maybe not.
    // pub fn construct1(pts: Vec<i32>) -> Node {
    //     if pts.len() == 0 || pts.len() % 2 == 1 {
    //         return (*get_zero(4).as_deref().unwrap()).clone()
    //     }

    //     let mut x_vals = Vec::new();
    //     let mut y_vals = Vec::new();
    //     for n in 0..pts.len() {
    //         if n % 2 == 0 {
    //             x_vals.push(pts[n]);
    //         }
    //         else {
    //             y_vals.push(pts[n]);
    //         }
    //     }
    //     let x_min = x_vals.iter().min().unwrap();
    //     let y_min = y_vals.iter().min().unwrap();

    //     let mut pattern: HashMap<(i32, i32), Option<Arc<Node>>> = std::collections::HashMap::new();
    //     let mut k = 0;

    //     while pattern.len() != 1 {
    //         let next_level: HashMap<(i32, i32), Option<Arc<Node>>> = std::collections::HashMap::new();
    //         let z = get_zero(k);

    //         while pattern.len() > 0 {
    //             let (mut x, mut y) = pattern.iter().next().unwrap().0;
    //             x = x - (x & 1);
    //             y = y - (y & 1);


    //         }
    //     }
    // }

    pub fn advance(mut node: Node, mut n: u32) -> Node {
        if n == 0 {
            return node;
        }
    
        let mut bits = Vec::new();
        while n > 0 {
            bits.push(n & 1);
            n >>= 1;
            node = (*center(Some(Arc::new(node))).as_deref().unwrap()).clone();
        }
    
        for (k, bit) in bits.iter().rev().enumerate() {
            let j: u8 = (bits.iter().len() - k - 1).try_into().unwrap();
            if bit != &0 {
                node = (*successor(Some(Arc::new(node)), Some(j)).unwrap()).clone();
            }
        }
        log(format!("{:?}", CALL_COUNT.load(Ordering::SeqCst)).as_str());
        CALL_COUNT.store(0, Ordering::SeqCst);

        // return node;
        return (*crop(Some(Arc::new(node))).as_deref().unwrap()).clone()
    }
    
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