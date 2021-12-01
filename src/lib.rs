#![feature(generators, generator_trait)]

mod gen_wrapper;

pub use gen_wrapper::*;
pub use genutils_macro::generator;

#[macro_export]
macro_rules! yield_from {
    ($gen_expr:expr) => {
        {
            use std::ops::{ Generator, GeneratorState };
            use std::pin::Pin;

            let mut __gen = $gen_expr;
            while let GeneratorState::Yielded(v) = Pin::new(&mut __gen).resume(()) {
                yield v;
            }
        }
    };
}