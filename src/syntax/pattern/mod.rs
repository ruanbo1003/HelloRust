#![allow(dead_code, unused_variables)]

mod pattern_match;
mod pattern_if;
mod pattern_while;

pub fn tests() {
    pattern_if::tests();

    pattern_match::tests();

    pattern_while::tests();

}

