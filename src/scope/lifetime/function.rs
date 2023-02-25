

/*
function with lifetime annotation:
    1. every reference must have lifetime annotation
    2. every returned reference must has a lifetime annotation, which is the same os input lifetime
    reference, or a `static annotation.
 */
pub fn tests() {

}

// mutable reference with lifetime annotation
fn add_one<'a>(x: &'a mut i32) {
    *x += 1
}

/*
return the input reference, the input lifetime and output lifetime
should match.
 */
fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    return x;
}

/*
return new string reference: error
 */
// fn new_string<'a>() -> &'a String {
//     &String::from("hello")
// }