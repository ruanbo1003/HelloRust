
pub fn tests() {

}

/*
 Return value can transfer ownership.
 */
fn return_value() {
    {
        let s = return_value_and_give_ownership();
        println!("get ownership from return value, s:{s}");  // hello-return
    }

    {
        let s1 = String::from("hello-take-and-back");
        println!("s1:{s1}");

        let s2 = take_ownership_and_give_back(s1);
        println!("s2:{s2}");
    }
}

fn return_value_and_give_ownership() -> String {
    let s = String::from("hello-return");
    println!("s in function:{s}");  // hello-return
    return s;
}

fn take_ownership_and_give_back(src: String) -> String {
    println!("src in function:{src}");
    return src;
}
