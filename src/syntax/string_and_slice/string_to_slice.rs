
use crate::util::variable::get_type_name;

pub fn tests() {
    {
        let s1 = String::from("hello");

        let s2 = &s1[..];
        let s3 = &s1[0..1];
        let s4 = &s1[0..s1.len()];

        // &str, &str, &str
        println!("{}, {}, {}", get_type_name(s2), get_type_name(s3), get_type_name(s4));
        // hello, h, hello
        println!("{}, {}, {}", s2, s3, s4);

        let s5 = &s4[..];
        println!("{}, {}", get_type_name(s5), s5);  // &str, hello

        let s6 = &s1;
        println!("{}, {}", get_type_name(s6), s6);  // &alloc::string::String, hello
    }
}