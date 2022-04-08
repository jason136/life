#![feature(test)]

extern crate test;
extern crate life;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = life::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}