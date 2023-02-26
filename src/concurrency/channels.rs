
use std::thread;
use std::sync::mpsc;
use std::time::Duration;


pub fn tests() {
    // mpsc_one_producer_send_one_msg();

    // mpsc_one_producer_send_multiple_msg();

    mpsc_multi_producer_send_multiple_msg();

}

fn mpsc_one_producer_send_one_msg() {
    let (tx, rx) = mpsc::channel();
    let produce_handle = thread::spawn(move || {
        let val = String::from("I'm produce thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("main thread received:{}", received);
}

fn mpsc_one_producer_send_multiple_msg() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("thread"),
        ];

        for each_msg in msgs {
            tx.send(each_msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received_msg in rx {
        println!("{}", received_msg);
    }
}

fn mpsc_multi_producer_send_multiple_msg() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let msgs = vec![
            String::from("Hello"),
            String::from("thread-1"),
        ];

        for each_msg in msgs {
            tx1.send(each_msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("Hello"),
            String::from("thread-2"),
        ];

        for each_msg in msgs {
            tx.send(each_msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received_msg in rx {
        println!("{}", received_msg);
    }
}