
pub fn tests() {
    {
        let fn_closure = gen_fn_closure();
        fn_closure();  // hello
        fn_closure();  // hello
    }

    {
        let mut fn_mut_closure = gen_fn_mut_closure();
        fn_mut_closure();  // Hello
        fn_mut_closure();  // Hello
    }

    {
        let fn_once_closure = gen_fn_once_closure();
        fn_once_closure();

        // use of moved value `fn_once_closure`
        // fn_once_closure();  // error
    }
}

fn gen_fn_closure() -> impl Fn() {
    || println!("hello")
}

fn gen_fn_mut_closure() -> impl FnMut() {
    let text = "Hello".to_owned();
    return move || println!("{}", text);
}

fn gen_fn_once_closure() -> impl FnOnce() {
    || println!("Rust")
}