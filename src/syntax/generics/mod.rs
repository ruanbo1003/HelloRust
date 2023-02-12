#![allow(dead_code)]

mod extract_code;
mod generic_function;
mod generic_struct;
mod traits;
mod trait_bound_parameter;
mod trait_bound_return;

pub fn tests() {
    // extract_code::extract();

    // generic_function::tests();

    // generic_struct::tests();

    // traits::tests();

    // trait_bound_parameter::tests();

    trait_bound_return::tests();
}
