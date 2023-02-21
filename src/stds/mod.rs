#![allow(dead_code, unused_variables)]

mod iterator_any;
mod iterator_find;

pub fn tests() {
    iterator_any::tests();

    iterator_find::tests();
}