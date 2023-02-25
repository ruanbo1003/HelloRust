
use std::thread;
use std::time::Duration;


pub fn tests() {
    create_new_thread();
}

fn create_new_thread() {
    let data = vec![1, 2, 3, 4];

    let thread_fun = move || {
        println!("data in thread:{:?}", data);

        for i in 1..10 {
            println!("thread - count:{}", i);
            thread::sleep(Duration::from_secs(1));
        }
    };
    let thread_handle = thread::spawn(thread_fun);
    thread_handle.join().unwrap();

    // error: borrow of moved value: `data`.
    // println!("data in main:{:?}", data);  // error

    for i in 1..5 {
        println!("main thread - count:{}", i);
    }

}