// allow non-snake-case for crate name(HelloRust)
#![allow(non_snake_case)]

extern crate core;

mod examples;
mod syntax;
mod scope;
mod util;
mod exercise;
mod stds;


fn main() {
    // examples::run_examples();

    // syntax::basic_syntax();

    scope::tests();

    // stds::tests();

    // exercise::tests();

    println!("\n== Hello, Rust! ==");
}
