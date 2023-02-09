#![allow(dead_code, unused_variables)]

mod basic;
mod variable_data_interact;
mod ownership_with_function;
mod ownership_with_return_value;


pub fn tests() {
    // basic::ownership_tests();

    // variable_data_interact::tests();

    // ownership_with_function::tests();

    ownership_with_return_value::tests();
}