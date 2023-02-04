#![allow(dead_code)]

pub fn tests() {
    // create a String
    {
        // create a empty string from String::new()
        let s1: String = String::new();
        println!("s1:{}", s1);

        // create a non-empty string from String::from()
        let s2 = String::from("hello");
        println!("s2:{s2}");

    }

    // update a String
    {
        let mut s1 = String::from("hello");
        s1.push_str(" world");
        println!("s1:{s1}");  // hello world

        let mut s2 = String::from("hell");
        s2.push('o');
        println!("s2:{s2}");  // hello
    }

    // operation +
    {
        let s1 = String::from("hello");
        let s2 = String::from(" world");
        let s3 = s1 + &s2;  // fn add(self, s: &str) -> String {}
        println!("s3:{s3}");  // hello world
        println!("s2:{s2}");  //  world
        // println!("s1:{s1}");  // error: borrow of moved value: `s1`
    }

    // concatenate multi strings using +
    {
        let s1 = String::from("A");
        let s2 = String::from("B");
        let s3 = String::from("C");
        let abc = s1 + "-" + &s2 + "-" +  &s3;
        println!("abc:{abc}");  // A-B-C
    }

    // concatenate multi strings using format!
    {
        let s1 = String::from("A");
        let s2 = String::from("B");
        let s3 = String::from("C");
        let abc = format!("{}-{}-{}", s1, s2, s3);
        println!("abc:{abc}");  // A-B-C
    }

    // indexing into Strings
    {
        let s = String::from("hello");
        println!("s:{}", s);
        // let h = s[0];  // error: the type `String` cannot be indexed by `{integer}`
        // println!("h:{}", h);
    }

    // string length
    {
        let s1 = String::from("hello");
        println!("len(s1):{}", s1.len());  // 5

        let s2 = String::from("你好");
        println!("len(s2):{}", s2.len());  // 6
    }

    // slice strings
    {
        let s1 = String::from("Hello");
        let h1 = &s1[0..2];
        println!("he:{}", h1);  // He
        // let h2 = s1[0..2];  // error: the size for values of type `str` cannot be known at compilation time

        let s2 = String::from("你好");
        let h3 = &s2[0..3];
        println!("h3:{}", h3);  // 你
        // let h4 = &s2[0..1];  // error: 'byte index 1 is not a char boundary; it is inside '你' (bytes 0..3) of `你好`'
    }

    // iterator strings - chars()
    {
        let s1 = "hello";
        for c in s1.chars() {
            print!("{}", c);
        }  // hello
        println!();

        let s2 = "你好";
        for c in s2.chars() {
            print!("{}", c);
        }  // 你好
        println!();
    }

    // iterator strings - bytes()
    {
        let s1 = String::from("hello");
        for b in s1.bytes() {
            print!("{} ", b);
        } // 104 101 108 108 111
        println!();
    }
}

