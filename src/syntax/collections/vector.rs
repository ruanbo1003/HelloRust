#![allow(dead_code)]

pub fn tests() {
    // create_a_new_vector();

    // vector_ownership();

    // vector_iterator();

    vector_multi_types();
}

fn create_a_new_vector() {
    {
        let v: Vec<i32> = Vec::new();
        println!("v:{:?}", v);  // []
    }

    {
        let v = vec![1, 2, 3];
        println!("v:{:?}", v);  // [1, 2, 3]
    }

    // update vector: push
    {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);

        println!("v:{:?}", v);  // [1, 2, 3]
    }

    // read element of vector
    {
        let v = vec![1, 2, 3, 4, 5];
        let third = &v[2];
        println!("third:{}", third);  // 3

        let third: Option<&i32> = v.get(2);
        println!("third:{:?}", third); // Some(3)

        match third {
            Some(third) => println!("third element is:{}", third),
            None => println!("There is no third element"),
        }
    }

    // read element of vector: index out of range
    {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];
        // let i1 = v[100];  // panic
        let i2: Option<&i32> = v.get(100);
        match i2 {
            Some(val) => println!("100th element:{}", val),
            None => println!("100th element not exists"),  // here
        }

        if let Some(val) = v.get(100) {
            println!("100th element:{}", val);
        } else {
            println!("100th element not exists"); // here
        }

        if let Some(val) = v.get(2) {
            println!("third element is:{}", val);  // 3
        } else {
            println!("third element not exists");
        }
    }
}

fn vector_ownership() {
    // ok
    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        println!("first element:{}", first);

        v.push(6);
        println!("v:{:?}", v);  // [1, 2, 3, 4, 5, 6]
    }

    // error
    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];

        // v.push(6);  // Error: cannot borrow `v` as mutable because it is also borrowed as immutable

        println!("first element:{}", first);

        v.push(6);
    }
}

fn vector_iterator() {
    {
        let v = vec![1, 2, 3];
        for i in &v {
            println!("{}", i);
        }
    }

    {
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i *= 10;
        }
        println!("after update v:{:?}", v); // [10, 20, 30]
    }
}

#[derive(Debug)]
enum  CellType {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_multi_types() {
    // use Enum to store multiple types

    {
        let v = vec![
            CellType::Int(1),
            CellType::Float(10.1),
            CellType::Text(String::from("hello")),
        ];
        println!("v:{:?}", v); // [Int(1), Float(10.1), Text("hello")]
    }
}