// allow non-snake-case for crate name(HelloRust)
#![allow(non_snake_case)]

mod examples;
mod syntax;
mod util;


fn main() {
    // examples::run_examples();

    syntax::basic_syntax();

    println!("\n== Hello, Rust! ==");
}
