// allow non-snake-case for crate name(HelloRust)
#![allow(non_snake_case)]

extern crate core;

mod examples;
mod syntax;
mod scope;
mod util;
mod exercise;
mod stds;
mod concurrency;


fn main() {
    // examples::run_examples();

    syntax::tests();

    // scope::tests();

    // stds::tests();

    // concurrency::tests();

    // exercise::tests();

    println!("\n== Hello, Rust! ==");
}
