#![allow(dead_code)]

mod variables;
mod functions;
mod comments;
mod control_flow;

mod structs;
mod enums;
mod collections;
mod errors;

mod generics;
mod closures;
mod iterator;
mod smart_pointer;
mod ownerships;
mod string_and_slice;
mod lifetime;
mod format;


pub fn basic_syntax() {
    // closures::tests();

    generics::tests();

    // format::tests();

    // variables::tests();

    // functions::basic_functions();

    // comments::comments();

    // control_flow::tests();

    // ownerships::ownership_tests();

    // structs::struct_tests();

    // enums::tests();

    // collections::tests();

    // errors::error_tests();

    // lifetime::tests();

    // iterator::tests();

    // ownerships::tests();

    // structs::tests();

    // string_and_slice::tests();

    // smart_pointer::tests();

    println!("end of rust basic syntax");
}