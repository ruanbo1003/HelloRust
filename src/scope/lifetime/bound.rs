
/*
Just like generic can be bounded, lifetime can use bound too. ( lifetime itself is a type of generic).
    1. T: 'a : for all reference in T, they must live longer then 'a
    2. T: Trait + 'a : T must implemented `Trait`, and all reference in T must live longer then 'a
 */

use std::fmt::Debug;

pub fn tests() {
    let x = 1;
    let ref_x = Ref(&x);

    print_ref(&ref_x);  // t: Ref(1)
    my_print(ref_x);  // t: Ref(1)
}

/*
`Ref` has a reference of type T.
T has lifetime limit, all reference in T must live longer than 'a.
 */
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

/*
a generic function.
 */
fn my_print<T>(t: T) where
    T: Debug {
    println!("t: {:?}", t);
}

/*
function parameter: reference type of T, T must implement the `Debug` trait.
All references in T must live longer then 'a, and 'a must live longer than
function `print_ref`.
 */
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("t: {:?}", t);
}
