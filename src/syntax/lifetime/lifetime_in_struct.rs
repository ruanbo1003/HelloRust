
pub fn tests() {
    lifetime_struct_declare();
}


/*
Define a struct with reference field.
We must add lifetime-annotation for the references.

The scope of `LfTest` instance, cannot be longer than then
variable which the field `name` referenced to.
 */
#[derive(Debug)]
struct LfTest<'a> {
    name: &'a str,  // string slice, reference
}

fn lifetime_struct_declare() {
    // OK: the struct reference field scope is smaller than the variable that referenced.
    {
        let s1 = "Hello, world";
        let first_str = s1.split(',')
            .next()
            .expect("',' not in s1");

        // the valid-scope of `lf` is smaller than `s1`
        let lf = LfTest{name: first_str};
        println!("{:?}", lf);  // LfTest { name: "Hello" }
    }

    // Failed: the struct reference field scope is bigger than the variable that referenced.
    {
        let mut lf = LfTest{ name: "test"};
        {
            let s1 = String::from("Hello");
            let first_str = s1.split(',')
                .next()
                .expect("',' not in s1");

            // the valid-scope of `lf` is bigger than `s1`
            lf.name = first_str;
            println!("{:?}", lf);  // LfTest { name: "Hello" }
        }  // `s1` dropped here while still borrowed.

        // error: `s1` does not live long enough.
        // println!("{:?}", lf);  // borrowed later used here.
    }
}
