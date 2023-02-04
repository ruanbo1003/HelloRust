#![allow(dead_code)]

use std::fs::File;
use std::io::{ErrorKind};

pub fn tests() {
    return_result();

}

fn return_result() {

    {
        let greeting_file_result = File::open("not-exists.txt");
        println!("ret:{:?}", greeting_file_result);

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("open file failed:{:?}", error),
        };
        println!("greeting_file:{:?}", greeting_file);
    }

    {
        let greeting_file_result = File::open("not-exists.txt");
        println!("ret:{:?}", greeting_file_result);

        let file2 = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("not-exists.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("create file failed:{:?}", e),
                },
                other_error => {
                    panic!("open file failed:{:?}", other_error);
                }
            }
        };
        println!("file2:{:?}", file2);
    }
}