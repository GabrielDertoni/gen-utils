#![feature(generators, generator_trait)]

use std::ops::Generator;
use genutils::{generator, yield_from};

#[generator]
fn gen2() -> impl Generator<Yield = u32> {
    for i in 0..10 {
        yield i;
    }
}

#[generator]
fn gen() -> impl Generator<Yield = u32> {
    yield 1;
    yield 2;
    yield_from!(gen2());
    yield_from!(|| {
        for i in 10..20 {
            yield i;
        }
    });
}

fn main() {}