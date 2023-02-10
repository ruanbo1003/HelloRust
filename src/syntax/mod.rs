
mod variables;
mod data_types;
mod functions;
mod comments;
mod control_flow;

mod structs;
mod enums;
mod collections;
mod errors;

mod generics;
mod closures;
mod smart_pointer;
mod ownerships;


pub fn basic_syntax() {
    // variables::variables();

    // data_types::data_types();

    // functions::basic_functions();

    // comments::comments();

    // control_flow::basic_control_flow();

    // ownerships::ownership_tests();

    // structs::struct_tests();

    // enums::enum_tests();

    // collections::tests();

    // errors::error_tests();

    // generics::tests();

    // closures::tests();

    ownerships::tests();

    println!("end of rust basic syntax");
}