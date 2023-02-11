
pub fn tests() {
    // regular_struct();

    // tuple_struct();

    empty_struct();
}

// ====== regular struct ======
fn regular_struct() {
    {
        let p1 = PersonWithNoCopy {
            age: 10,
            name: "Tom".to_string(),
        };
        println!("{:?}", p1);  // PersonWithNoCopy { age: 10, name: "Tom" }

        // declare mutable struct variable, and update the field value.
        let mut p2 = PersonWithNoCopy {
            age: 10,
            name: "Jack".to_string(),
        };
        p2.age = 12;
        println!("{:?}", p2);  // PersonWithNoCopy { age: 12, name: "Tom" }
    }

    {
        let name = "jack".to_string();
        let p = gen_no_copy_person_by_name(name);
        println!("{:?}", p);  // PersonWithNoCopy { age: 10, name: "jack" }

        // error
        // println!("{}", name);  // borrow of moved value: `name`
    }

    // create a PersonWithNoCopy from another PersonWithNoCopy
    {
        {
            let p1 = gen_no_copy_person_by_name(String::from("tom"));
            println!("{:?}", p1);  // PersonWithNoCopy { age: 10, name: "tom" }

            let p2 = gen_no_copy_person_by_another(p1);
            println!("{:?}", p2);  // PersonWithNoCopy { age: 100, name: "tom" }

            // error
            // println!("{:?}", p1);  // borrow of moved value: `p1`
        }

        {
            let p1 = PersonWithCopy {
                age: 20,
                height: 100,
            };
            println!("{:?}", p1);  // PersonWithCopy { age: 20, height: 100 }

            let p2 = gen_copy_person_by_another(p1);
            println!("{:?}", p2); // PersonWithCopy { age: 101, height: 100 }

            //because PersonWithCopy is copyable, so p1 is valid here.
            println!("{:?}", p1);  // PersonWithCopy { age: 20, height: 100 }
        }
    }
}


#[derive(Debug)]
struct PersonWithNoCopy {
    age: i32,
    name: String,
}

#[derive(Debug, Copy, Clone)]
struct PersonWithCopy {
    age: i32,
    height: i32,
}

fn gen_no_copy_person_by_name(name: String) -> PersonWithNoCopy {
    return PersonWithNoCopy{
        name,  // field name is the same as parameter,
        age: 10,
    };
}

fn gen_no_copy_person_by_another(person: PersonWithNoCopy) ->PersonWithNoCopy {
    return PersonWithNoCopy {
        age: 100,  // we assign the value of age here
        ..person  // the rest fields value will come from parameter `person`
    };
}

fn gen_copy_person_by_another(person: PersonWithCopy) -> PersonWithCopy {
    return PersonWithCopy {
        age: 101,
        ..person
    };
}

// ======= tuple struct ========
fn tuple_struct() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);

    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let c = Color(1, 2, 3);
    println!("{:?}", c);  // Color(1, 2, 3)
    println!("{}, {}, {}", c.0, c.1, c.2); // 1, 2, 3

    let p = Point(1, 2, 3);
    println!("{:?}", p);  // Point(1, 2, 3)
    println!("{}, {}, {}", p.0, p.1, p.2)  // 1, 2, 3
}

// ===== empty struct ====
fn empty_struct() {
    #[derive(Debug)]
    struct Empty1;

    #[derive(Debug)]
    struct Empty2;

    let e1 = Empty1{};
    println!("{:?}", e1);  // Empty1

    let e2 = Empty2{};
    let e3 = Empty2;
    println!("{:?}", e2);  // Empty2
    println!("{:?}", e3);  // Empty2


}