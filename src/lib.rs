use wasm_bindgen::prelude::*;

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
    static ref JOINCACHE: Mutex<HashMap<Vec<u64>, Option<Arc<Node>>>> = {
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
    // fn a(&self) -> Option<Arc<Node>> {
    //     self.a
    // }
    // fn b(&self) -> Option<Arc<Node>> {
    //     *self.b.as_ref().unwrap().clone()
    // }
    // fn c(&self) -> Option<Arc<Node>> {
    //     *self.c.as_ref().unwrap().clone()
    // }
    // fn d(&self) -> Option<Arc<Node>> {
    //     *self.d.as_ref().unwrap().clone()
    // }
}

fn join(a: Option<Arc<Node>>, b: Option<Arc<Node>>, c: Option<Arc<Node>>, d: Option<Arc<Node>>) -> Option<Arc<Node>> {
    let hash_vec = vec![a.hash(), b.hash(), c.hash(), d.hash()];
    if a.level() > 6 && JOINCACHE.lock().unwrap().contains_key(&hash_vec) {
        let n = JOINCACHE.lock().unwrap().get(&hash_vec).unwrap().clone();
        return n;
    }

    let n_level = &a.level() + 1;
    let n_population: u32 = &a.population() + &b.population() + &c.population() + &d.population();
    let n_hash: u64 = (
        &a.hash() * 5131830419411 +
        &b.hash() * 3758991985019 +
        &c.hash() * 8973110871315 +
        &d.hash() * 4318490180473 +
        u64::from(a.level())
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

    JOINCACHE.lock().unwrap().insert(hash_vec, n.clone());
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
    let zero = get_zero(m.level() - 1);
    join (
        join(zero.clone(), zero.clone(), zero.clone(), m.a()),
        join(zero.clone(), zero.clone(), m.b(),zero.clone()),
        join(zero.clone(), m.c(), zero.clone(),zero.clone()),
        join(m.d(), zero.clone(), zero.clone(),zero),
    )
}

fn life(a: &Option<Arc<Node>>, b: &Option<Arc<Node>>, c: &Option<Arc<Node>>, d: &Option<Arc<Node>>, e: &Option<Arc<Node>>, 
        f: &Option<Arc<Node>>, g: &Option<Arc<Node>>, h: &Option<Arc<Node>>, i: &Option<Arc<Node>>) -> Option<Arc<Node>> {
    let mut outer = 0;
    for n in [a, b, c, d, f, g, h, i].iter() {
        if n.population() > 0 {
            outer += 1;
        }
    }
    match outer {
        3 => Some(Arc::new(ON.clone())),
        2 => {
            if e.population() > 0 && outer == 2 {
                Some(Arc::new(ON.clone()))
            }
            else {
                Some(Arc::new(OFF.clone()))
            }
        }
        _ => Some(Arc::new(OFF.clone()))
    }
}
fn life_4x4(m: &Option<Arc<Node>>) -> Option<Arc<Node>> {
    let a = m.a();
    let b = m.b();
    let c = m.c();
    let d = m.d();

    let ab = life(&a.a(), &a.b(), &b.a(), &a.c(),&a.d(), &b.c(), &c.a(), &c.b(), &d.a());
    let bc = life(&a.b(), &b.a(), &b.b(), &a.d(),&b.c(), &b.d(), &c.b(), &d.a(), &d.b());
    let cb = life(&a.c(), &a.d(), &b.c(), &c.a(),&c.b(), &d.a(), &c.c(), &c.d(), &d.c());
    let da = life(&a.d(), &b.c(), &b.d(), &c.b(),&d.a(), &d.b(), &c.d(), &d.c(), &d.d());

    join(
        ab,
        bc,
        cb,
        da,
    )
}

fn successor(m: Option<Arc<Node>>, j: Option<u8>) -> Option<Arc<Node>> {
    if m.level() == 0 {
        return m.a().clone()
    }
    if m.level() == 2 {
        return life_4x4(&m)
    }

    if SUCCESSORCACHE.lock().unwrap().contains_key(&(m.hash(), j)) {
        return SUCCESSORCACHE.lock().unwrap().get(&(m.hash(), j)).unwrap().clone();
    }
    
    let nj: Option<u8> = if j.is_none() {
        Some(m.level() - 2)
    }
    else {
        Some(std::cmp::min(j.unwrap(), m.level() - 2))
    };

    let a = m.a();
    let b = m.b();
    let c = m.c();
    let d = m.d();

    let c1 = successor(join(a.a(), a.b(), a.c(), a.d()), nj);
    let c2 = successor(join(a.b(), b.a(), a.d(), b.c()), nj);
    let c3 = successor(join(b.a(), b.b(), b.c(), b.d()), nj);
    let c4 = successor(join(a.c(), a.d(), c.a(), c.b()), nj);
    let c5 = successor(join(a.d(), b.c(), c.b(), d.a()), nj);
    let c6 = successor(join(b.c(), b.d(), d.a(), d.b()), nj);
    let c7 = successor(join(c.a(), c.b(), c.c(), c.d()), nj);
    let c8 = successor(join(c.b(), d.a(), c.d(), d.c()), nj);
    let c9 = successor(join(d.a(), d.b(), d.c(), d.d()), nj);

    let n: Option<Arc<Node>>;
    if nj.unwrap() < m.level() - 2 {
        n = join(
            join(c1.d(), c2.c(), c4.b(), c5.a()),
            join(c2.d(), c3.c(), c5.b(), c6.a()),
            join(c4.d(), c5.c(), c7.b(), c8.a()),
            join(c5.d(), c6.c(), c8.b(), c9.a())
        );
    }
    else {
        n = join(
            successor(join(c1, c2.clone(), c4.clone(), c5.clone()), nj),
            successor(join(c2, c3, c5.clone(), c6.clone()), nj),
            successor(join(c4, c5.clone(), c7, c8.clone()), nj),
            successor(join(c5, c6, c8, c9), nj)
        );
    }

    SUCCESSORCACHE.lock().unwrap().insert((n.hash(), j), n.clone());
    return n;
}

fn advance(mut node: Option<Arc<Node>>, mut n: u32) -> Option<Arc<Node>> {
    if n == 0 {
        return node;
    }

    let mut bits = Vec::new();
    while n > 0 {
        bits.push(n & 1);
        n >>= 1;
        node = center(node);
    }

    for (k, bit) in bits.iter().rev().enumerate() {
        let j: u8 = (bits.iter().len() - k - 1).try_into().unwrap();
        if bit != &0 {
            node = successor(node, Some(j));
        }
    }
    node
}

#[wasm_bindgen]
impl Node {
    // not really used
    pub fn advance(mut node: Node, mut n: u32) -> Node {
        if n == 0 {
            return node;
        }
    
        let mut bits = Vec::new();
        while n > 0 {
            bits.push(n & 1);
            n >>= 1;
            node = (*center(Some(Arc::new(node))).unwrap()).clone();
        }
    
        for (k, bit) in bits.iter().rev().enumerate() {
            let j: u8 = (bits.iter().len() - k - 1).try_into().unwrap();
            if bit != &0 {
                node = (*successor(Some(Arc::new(node)), Some(j)).unwrap()).clone();
            }
        }
        node
    }
    
    pub fn ffwd(node: Node, n: u32) -> Node {
        let mut node = Some(Arc::new(node));
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
        (*node.unwrap()).clone()
    }

    pub fn expand(node: &Node, x: i32, y: i32) -> Vec<i32> {
        if node.population() == 0 {
            return Vec::new()
        }

        let size = u32::pow(2, node.level().into());

        if node.level() == 0 {
            return vec![x, y]
        }
        else {
            let offset = (size >> 1) as i32;
            let mut new = Vec::new();
            new.append(&mut Node::expand(&node.a.as_ref().unwrap(), x, y));
            new.append(&mut Node::expand(&node.b.as_ref().unwrap(), x + offset, y));
            new.append(&mut Node::expand(&node.c.as_ref().unwrap(), x, y + offset));
            new.append(&mut Node::expand(&node.d.as_ref().unwrap(), x + offset, y + offset));
            new
        }
    }

    pub fn construct(pts: Vec<i32>) -> Node {
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
        (**pattern[&last_updated].as_ref().unwrap()).clone()
    }
}

// https://johnhw.github.io/hashlife/index.md.html