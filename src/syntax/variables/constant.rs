

/*
there are two constant in Rust, can be declared in any scope.

const: un-changeable value
static: has a lifetime of 'static, mutable variable(when use `static mut`)
 */


pub fn tests() {
    const ONE_HOUR: i32 = 1 * 60 * 60;
    println!("one hour:{}", ONE_HOUR);  // 3600

    static LANGUAGE: &'static str = "Rust";

    println!("{}", LANGUAGE);

}
