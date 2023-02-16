// allow non-snake-case for crate name(HelloRust)
#![allow(non_snake_case)]

extern crate core;

mod examples;
mod syntax;
mod util;


fn main() {
    // examples::run_examples();

    syntax::basic_syntax();

    // exercise::tests();

    println!("\n== Hello, Rust! ==");
}
