#![allow(dead_code, unused_variables)]

mod raii;
mod ownership_and_move;
mod borrow;
mod lifetime;

pub fn tests() {
    // raii::tests();

    // ownership_and_move::tests();

    // borrow::tests();

    lifetime::tests();
}