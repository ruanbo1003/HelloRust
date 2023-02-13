#![allow(dead_code, unused_variables)]

mod iterators;
mod my_iterator;

pub fn tests() {
    iterators::tests();

    my_iterator::tests();
}