mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

fn center(m: &Node) -> Node {
    let zero = &get_zero(m.level() - 1);
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
    else if m.level() == 2 {
        life_4x4(m)
    }
    else {
        let nj: Option<u32>;
        if j.is_none() {
            nj = Some(m.level() - 2);
        }
        else {
            nj = Some(std::cmp::min(j.unwrap(), m.level() - 2));
        }

        let c1 = successor(&join(&a.a(), &a.b(), &a.c(), &a.d()), nj);
        let c2 = successor(&join(&a.b(), &b.a(), &a.d(), &b.c()), nj);
        let c3 = successor(&join(&b.a(), &b.b(), &b.c(), &b.d()), nj);
        let c4 = successor(&join(&a.c(), &a.d(), &c.a(), &c.b()), nj);
        let c5 = successor(&join(&a.d(), &b.c(), &c.b(), &d.a()), nj);
        let c6 = successor(&join(&b.c(), &b.d(), &d.a(), &d.b()), nj);
        let c7 = successor(&join(&c.a(), &c.b(), &c.c(), &c.d()), nj);
        let c8 = successor(&join(&c.b(), &d.a(), &c.d(), &d.c()), nj);
        let c9 = successor(&join(&d.a(), &d.b(), &d.c(), &d.d()), nj);

        if j.unwrap() < m.level() - 2 {
            join(
                &join(&c1.d(), &c2.c(), &c4.b(), &c5.a()),
                &join(&c2.d(), &c3.c(), &c5.b(), &c6.a()),
                &join(&c4.d(), &c5.c(), &c7.b(), &c8.a()),
                &join(&c5.d(), &c6.c(), &c8.b(), &c9.a())
            )
        }
        else {
            join(
                &successor(&join(&c1.d(), &c2.c(), &c4.b(), &c5.a()), nj),
                &successor(&join(&c2.d(), &c3.c(), &c5.b(), &c6.a()), nj),
                &successor(&join(&c4.d(), &c5.c(), &c7.b(), &c8.a()), nj),
                &successor(&join(&c5.d(), &c6.c(), &c8.b(), &c9.a()), nj)
            )
        }
    }
}

fn advance(node: &Node, mut n: u32) -> Node {
    let mut node = node.clone();
    if n == 0 {
        return node.clone();
    }

    let mut bits = Vec::new();
    while n > 0 {
        bits.push(n & 1);
        n = n >> 1;
        node = center(&node);
    }

    for (k, bit) in bits.iter().rev().enumerate() {
        let j: u32 = (bits.iter().len()- k - 1).try_into().unwrap();
        if bit != &0 {
            node = successor(&node, Some(j));
        }
    }
    node
}

fn ffwd(node: &Node, n: u32) -> Node {
    let mut node = node.clone();
    for _ in 0..n {
        while node.level() < 3 || 
            node.a().level() != node.a().d().d().level() ||
            node.b().level() != node.b().c().c().level() ||
            node.d().level() != node.d().a().a().level() {
            node = center(&node);
        }
        node = successor(&node, None);
    }
    node
}

#[wasm_bindgen]
impl Node {
    pub fn expand(node: &Node, x: i32, y: i32) -> Vec<i32> {
        if node.population() == 0 {
            return Vec::new()
        }

        let size = u32::pow(2, node.level());

        if node.level() == 0 {
            return vec![x, y]
        }
        else {
            let offset = (size >> 1) as i32;
            let mut new = Vec::new();
            new.append(&mut Node::expand(&node.a(), x, y));
            new.append(&mut Node::expand(&node.b(), x + offset, y));
            new.append(&mut Node::expand(&node.c(), x, y + offset));
            new.append(&mut Node::expand(&node.d(), x + offset, y + offset));
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
                ON.clone()
            );
        }

        let mut k = 0;
        let mut last_updated: Node = ON.clone();
        while pattern.len() != 1 {
            let mut next_level = std::collections::HashMap::new();
            let z = get_zero(k);

            while pattern.len() > 0 {
                let next_pair = pattern.iter().next().unwrap();
                let mut x = next_pair.0.0;
                let mut y = next_pair.0.1;
                x = x - (x & 1);
                y = y - (y & 1);

                let (a, b, c, d): (Node, Node, Node, Node);
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

                next_level.insert(
                    (x >> 1, y >> 1),
                    join(&a, &b, &c, &d)
                );
                last_updated = next_level.get(&(x >> 1, y >> 1)).unwrap().clone();
            }
            pattern = next_level;
            k += 1;
        }
        last_updated
    }
}

// https://johnhw.github.io/hashlife/index.md.html