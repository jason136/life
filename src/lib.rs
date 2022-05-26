mod utils;
#[cfg(feature = "console_error_panic_hook")]
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
    // fn a(&self) -> Option<Box<Node>> {
    //     self.a
    // }
    // fn b(&self) -> Option<Box<Node>> {
    //     *self.b.as_ref().unwrap().clone()
    // }
    // fn c(&self) -> Option<Box<Node>> {
    //     *self.c.as_ref().unwrap().clone()
    // }
    // fn d(&self) -> Option<Box<Node>> {
    //     *self.d.as_ref().unwrap().clone()
    // }
}

fn join(a: Option<Box<Node>>, b: Option<Box<Node>>, c: Option<Box<Node>>, d: Option<Box<Node>>) -> Option<Box<Node>> {
    let n_level = &a.unwrap_ref().level + 1;
    let n_population: u32 = &a.unwrap_ref().population + &b.unwrap_ref().population + &c.unwrap_ref().population + &d.unwrap_ref().population;
    let n_hash: u64 = (
        &a.unwrap_ref().hash * 5131830419411 +
        &b.unwrap_ref().hash * 3758991985019 +
        &c.unwrap_ref().hash * 8973110871315 +
        &d.unwrap_ref().hash * 4318490180473 +
        u64::from(a.unwrap_ref().level)
    ) & ((1 << 63) - 1);

    Some(Box::new(Node {
        a,
        b,
        c,
        d,
        population: n_population,
        level: n_level,
        hash: n_hash,
    }))
}

fn get_zero(k: u32) -> Option<Box<Node>> {
    if k == 0 {
        Some(Box::new(OFF.clone()))
    }
    else {
        join (
            get_zero(k - 1),
            get_zero(k - 1),
            get_zero(k - 1),
            get_zero(k - 1),
        )
    }
}

fn center(m: Option<Box<Node>>) -> Option<Box<Node>> {
    let zero = get_zero(m.unwrap_ref().level - 1);
    join (
        join(zero.clone(), zero.clone(), zero.clone(), m.unwrap_ref().a.clone()),
        join(zero.clone(), zero.clone(), m.unwrap_ref().b.clone(),zero.clone()),
        join(zero.clone(), m.unwrap_ref().c.clone(), zero.clone(),zero.clone()),
        join(m.unwrap_ref().d.clone(), zero.clone(), zero.clone(),zero.clone()),
    )
}

fn life(a: &Option<Box<Node>>, b: &Option<Box<Node>>, c: &Option<Box<Node>>, d: &Option<Box<Node>>, e: &Option<Box<Node>>, 
        f: &Option<Box<Node>>, g: &Option<Box<Node>>, h: &Option<Box<Node>>, i: &Option<Box<Node>>) -> Option<Box<Node>> {
    let mut outer = 0;
    for n in [a, b, c, d, f, g, h, i].iter() {
        if n.unwrap_ref().population > 0 {
            outer += 1;
        }
    }
    match outer {
        3 => Some(Box::new(ON.clone())),
        2 => {
            if e.unwrap_ref().population > 0 && outer == 2 {
                Some(Box::new(ON.clone()))
            }
            else {
                Some(Box::new(OFF.clone()))
            }
        }
        _ => Some(Box::new(OFF.clone()))
    }
}
fn life_4x4(m: Option<Box<Node>>) -> Option<Box<Node>> {
    let a = m.unwrap_ref().a.unwrap_ref();
    let b = m.unwrap_ref().b.unwrap_ref();
    let c = m.unwrap_ref().c.unwrap_ref();
    let d = m.unwrap_ref().d.unwrap_ref();

    let ab = life(&a.a, &a.b, &b.a, &a.c,&a.d, &b.c, &c.a, &c.b, &d.a);
    let bc = life(&a.b, &b.a, &b.b, &a.d,&b.c, &b.d, &c.b, &d.a, &d.b);
    let cb = life(&a.c, &a.d, &b.c, &c.a,&c.b, &d.a, &c.c, &c.d, &d.c);
    let da = life(&a.d, &b.c, &b.d, &c.b,&d.a, &d.b, &c.d, &d.c, &d.d);

    join(
        ab,
        bc,
        cb,
        da,
    )
}

fn successor(m: Option<Box<Node>>, j: Option<u32>) -> Option<Box<Node>> {
    //log("successor");
    if m.unwrap_ref().level == 0 {
        m.unwrap_ref().a.clone()
    }
    else if m.unwrap_ref().level == 2 {
        life_4x4(m)
    }
    else {
        let nj: Option<u32>;
        if j.is_none() {
            nj = Some(m.unwrap_ref().level - 2);
        }
        else {
            nj = Some(std::cmp::min(j.unwrap(), m.unwrap_ref().level - 2));
        }

        let a = m.unwrap_ref().a.unwrap_ref();
        let b = m.unwrap_ref().b.unwrap_ref();
        let c = m.unwrap_ref().c.unwrap_ref();
        let d = m.unwrap_ref().d.unwrap_ref();

        let c1 = successor(join(a.a.clone(), a.b.clone(), a.c.clone(), a.d.clone()), nj);
        let c2 = successor(join(a.b.clone(), b.a.clone(), a.d.clone(), b.c.clone()), nj);
        let c3 = successor(join(b.a.clone(), b.b.clone(), b.c.clone(), b.d.clone()), nj);
        let c4 = successor(join(a.c.clone(), a.d.clone(), c.a.clone(), c.b.clone()), nj);
        let c5 = successor(join(a.d.clone(), b.c.clone(), c.b.clone(), d.a.clone()), nj);
        let c6 = successor(join(b.c.clone(), b.d.clone(), d.a.clone(), d.b.clone()), nj);
        let c7 = successor(join(c.a.clone(), c.b.clone(), c.c.clone(), c.d.clone()), nj);
        let c8 = successor(join(c.b.clone(), d.a.clone(), c.d.clone(), d.c.clone()), nj);
        let c9 = successor(join(d.a.clone(), d.b.clone(), d.c.clone(), d.d.clone()), nj);

        if nj.unwrap() < m.unwrap().level - 2 {
            join(
                join(c1.unwrap_ref().d.clone(), c2.unwrap_ref().c.clone(), c4.unwrap_ref().b.clone(), c5.unwrap_ref().a.clone()),
                join(c2.unwrap_ref().d.clone(), c3.unwrap_ref().c.clone(), c5.unwrap_ref().b.clone(), c6.unwrap_ref().a.clone()),
                join(c4.unwrap_ref().d.clone(), c5.unwrap_ref().c.clone(), c7.unwrap_ref().b.clone(), c8.unwrap_ref().a.clone()),
                join(c5.unwrap_ref().d.clone(), c6.unwrap_ref().c.clone(), c8.unwrap_ref().b.clone(), c9.unwrap_ref().a.clone())
            )
        }
        else {
            join(
                successor(join(c1.clone(), c2.clone(), c4.clone(), c5.clone()), nj),
                successor(join(c2.clone(), c3.clone(), c5.clone(), c6.clone()), nj),
                successor(join(c4.clone(), c5.clone(), c7.clone(), c8.clone()), nj),
                successor(join(c5.clone(), c6.clone(), c8.clone(), c9.clone()), nj)
            )
        }
    }
}

#[wasm_bindgen]
impl Node {
    pub fn advance(node: Node, mut n: u32) -> Node {
        if n == 0 {
            return node;
        }
    
        let mut node = Some(Box::new(node));
        let mut bits = Vec::new();
        while n > 0 {
            bits.push(n & 1);
            n = n >> 1;
            node = center(node);
        }
    
        for (k, bit) in bits.iter().rev().enumerate() {
            let j: u32 = (bits.iter().len() - k - 1).try_into().unwrap();
            if bit != &0 {
                node = successor(node, Some(j));
            }
        }
        *node.unwrap()
    }
    
    pub fn ffwd(node: Node, n: u32) -> Node {
        let mut node = Some(Box::new(node));
        for _ in 0..n {
            while node.unwrap_ref().level < 3 || 
                node.unwrap_ref().a.unwrap_ref().population != node.unwrap_ref().a.unwrap_ref().d.unwrap_ref().d.unwrap_ref().population ||
                node.unwrap_ref().b.unwrap_ref().population != node.unwrap_ref().b.unwrap_ref().c.unwrap_ref().c.unwrap_ref().population ||
                node.unwrap_ref().c.unwrap_ref().population != node.unwrap_ref().c.unwrap_ref().b.unwrap_ref().b.unwrap_ref().population ||
                node.unwrap_ref().d.unwrap_ref().population != node.unwrap_ref().d.unwrap_ref().a.unwrap_ref().a.unwrap_ref().population {
                log("looping begin");
                node = center(node);
                log("looping end");
            }
            log("done looping");
            node = successor(node, None);
        }
        *node.unwrap()
    }

    pub fn expand(node: &Node, x: i32, y: i32) -> Vec<i32> {
        if node.population == 0 {
            return Vec::new()
        }

        let size = u32::pow(2, node.level);

        if node.level == 0 {
            return vec![x, y]
        }
        else {
            let offset = (size >> 1) as i32;
            let mut new = Vec::new();
            new.append(&mut Node::expand(&node.a.unwrap_ref(), x, y));
            new.append(&mut Node::expand(&node.b.unwrap_ref(), x + offset, y));
            new.append(&mut Node::expand(&node.c.unwrap_ref(), x, y + offset));
            new.append(&mut Node::expand(&node.d.unwrap_ref(), x + offset, y + offset));
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
                Some(Box::new(ON.clone()))
            );
        }

        let mut k = 0;
        let mut last_updated = (0, 0);
        while pattern.len() != 1 {
            let mut next_level = std::collections::HashMap::new();
            let z = get_zero(k);

            while pattern.len() > 0 {

                log("construction begin");

                let (mut x, mut y) = pattern.iter().next().unwrap().0;
                x = x - (x & 1);
                y = y - (y & 1);

                let (a, b, c, d): (Option<Box<Node>>, Option<Box<Node>>, Option<Box<Node>>, Option<Box<Node>>);
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

                log("weird step1");
                last_updated = (x >> 1, y >> 1);

                log("weird step2");
                next_level.insert((x >> 1, y >> 1), join(a, b, c, d));

                log(next_level.len().to_string().as_str());
            }
            pattern = next_level;
            k += 1;
        }
        log("begin copy");
        *pattern[&last_updated].unwrap_ref().clone()
    }
}

// https://johnhw.github.io/hashlife/index.md.html