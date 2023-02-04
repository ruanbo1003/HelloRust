#![allow(dead_code)]

mod methods;


pub fn struct_tests() {
    // struct_basic_tests();

    methods::struct_method_tests();

    println!("end of struct tests");
}

fn struct_basic_tests() {
    first_struct_instance();

    let name = String::from("name");
    let email = String::from("email");
    let user2 = build_user_instance(name, email);
    println!("user2: {}, {}, {}, {}", user2.username, user2.email, user2.active, user2.sign_in_count);

    build_user_from_another_instance();

    tuple_struct_test();

    use_of_struct();

    struct_format_test();
}

struct DefinedUser {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn first_struct_instance() {
    let user1 = DefinedUser {
        active: false,
        username: String::from("username_1"),
        email: String::from("1@a.coom"),
        sign_in_count: 0
    };

    println!("user1:{}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);
}

fn build_user_instance(username: String, email: String) -> DefinedUser {
    DefinedUser {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_from_another_instance() {
    {
        let user1 = DefinedUser {
            username: String::from("name1"),
            email: "email1".to_string(),
            active: true,
            sign_in_count: 0,
        };

        let user2 = DefinedUser {
            active: user1.active,
            username: user1.username,
            email: "email2".to_string(),
            sign_in_count: user1.sign_in_count,
        };

        println!("user2:{}", user2.email);
    }

    {
        let user1 = DefinedUser {
            username: String::from("name1"),
            email: "email1".to_string(),
            active: true,
            sign_in_count: 0,
        };

        let user2 = DefinedUser {
            email: "email2".to_string(),
            ..user1
        };

        println!("user2:{}", user2.email);
    }
}

fn tuple_struct_test() {
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black:{},{},{}", black.0, black.1, black.2); // 0, 0, 0

    struct Point(i32, i32, i32);
    let p1 = Point(1, 2, 3);
    println!("p1:{}, {}, {}", p1.0, p1.1, p1.2);  // 1, 2, 3
}



fn use_of_struct() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn rectangle_area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rectangle1 = Rectangle {
        width: 10,
        height: 2
    };
    let area = rectangle_area(&rectangle1);
    println!("area:{}", area);  // 20
}


struct NoDebugStruct {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct WithDebugStruct {
    width: u32,
    height: u32,
}

fn struct_format_test() {
    let s1 = NoDebugStruct {
        width: 1,
        height: 1,
    };
    // println!("s1:{:?}", s1);  // `NoDebugStruct` doesn't implement `Debug` (required by {:?})
    println!("s1:{}, {}", s1.width, s1.height);

    let s2 = WithDebugStruct {
        width: 1,
        height: 1,
    };
    println!("s2:{:?}", s2);  // WithDebugStruct { width: 1, height: 1 }

    let s3 = WithDebugStruct {
        width: dbg!(100),  // add dbg!
        height: 2,
    };
    println!("s3:{:?}", s3);
}