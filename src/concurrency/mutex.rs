
use std::thread;
use std::sync::{Arc, Mutex};

pub fn tests() {
    mutex_counter_test();

}

fn mutex_counter_test() {
    let counter = Arc::new(Mutex::new(0i32));
    let mut handlers = vec![];

    for _ in 0..10000 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handle);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("counter:{}", *counter.lock().unwrap());

}