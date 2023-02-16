#![allow(non_camel_case_types)]

/*
use `type` to define a alias of another type.
 */
pub fn tests() {
    type UnsignedInt64 = u64;
    type SignedInt32 = i32;

    let u1: UnsignedInt64 = 1;
    let i1: SignedInt32 = 2;
    println!("{}, {}", u1, i1);  // 1, 2
}
