/*
    Rust use explicit annotation to make sure what lifetime should be.

    fn foo<'a>(){} : `foo` has a lifetime parameter `'a`.

    fn foo<`a, `b>(){} : `foo` has two lifetime parameter `'a` and `'b`.
 */

pub fn tests() {
    {
        let (four, nine) = (4, 9);
        print_ref(&four, &nine);  // 4, 9
    }
}

fn print_ref<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("{}, {}", x, y);
}

fn foo<'a>() {
    let _x = 10;

    // y has lifetime of 'a, which is longer then _x (_x will be dropped when out-of the scope)
    // let y: &'a i32 = &_x; // error: `_x` does not live long enough.
}