
use crate::util::variable::get_type_name;


pub fn tests() {
    {
        let s = "hello";  // string literal `hello` is stored in binary.

        // the type of s is &str, it's a immutable reference,
        // pointed to the binary-stored string `hello`
        println!("{}", get_type_name(s));  // &str

        let s2 = String::from("world");
        println!("{}", get_type_name(s2));  // alloc::string::String
    }

    {
        let s1 = "hello";
        let s2 = "Hi中国";
        let s3 = String::from("Hi中国");

        println!("{},{}", s1.len(), s2.len());  // 5 8
        println!("{}", s3.len());  // 8

        // error
        // s1[0] = 'H'; // error: the type `str` cannot be indexed by `{integer}`
    }
}