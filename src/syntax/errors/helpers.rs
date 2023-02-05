#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::Read;

pub fn result_helpers() {
    // unwrap_helper();

    // expect_helper();

    let result = propagate_error();
    println!("result:{:?}", result);
}

fn unwrap_helper() {
    // If the Result value is Ok variant, unwrap will return the value inside the Ok.
    // If the Result value is Err variant, unwrap will call the panic! macro for us.
    let greeting_file = File::open("hello.txt").unwrap();
    println!("greeting_file:{:?}", greeting_file);
}

fn expect_helper() {
    let greeting_file = File::open("hello.txt").expect("open file failed");
    println!("greeting_file:{:?}", greeting_file);
}

fn propagate_error() -> Result<String, io::Error> {
    {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
}

fn propagate_error2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn propagate_error3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}