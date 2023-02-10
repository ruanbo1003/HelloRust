
pub fn tests() {
    // take the ownership of a String variable
    {
        let s = String::from("hello");
        take_ownership_of_string(s);

        // the ownership of x is gone, because passed to function `take_ownership_of_string`
        // println!("s:{}", s);  // error: borrow of moved value `s`
    }

    // not take ownership of a integer variable
    {
        let i = 10;

        // The type of i32 is `Copyable`.
        // When we pass `i` to the function `not_take_ownership_of_integer`,
        // Rust copy a variable and pass the copied-variable to function.
        not_take_ownership_of_integer(i);

        // unlike the variable `s` above, i is still valid.
        println!("i:{}", i);  // ok
    }
}

fn take_ownership_of_string(s: String) {
    println!("take_ownership_of_string:{}", s);
}  // s is out-of scope, and the call `Drop` function to free resources.


fn not_take_ownership_of_integer(i: i32) {
    println!("not_take_ownership_of_integer:{}", i);
}
