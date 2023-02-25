
use std::thread;
use std::sync::mpsc;


pub fn tests() {
    let (tx, rx) = mpsc::channel();
    let produce_handle = thread::spawn(move || {
        let val = String::from("I'm produce thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("main thread received:{}", received);
}