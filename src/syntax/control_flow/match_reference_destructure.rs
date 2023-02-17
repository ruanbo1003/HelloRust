

/*
destructor: &, ref, ref mut
dereference: *
 */

pub fn tests() {
    {
        // get a reference of type `i32`; `&` operator means get reference.
        // the type of `reference` is `&i32`
        let reference = &4;

        match reference {
            // `&i32` <--> `&val`  <--> type of `val` is `i32`
            &val => {
                println!("val:{}", val); // 4
            }
        }

        // if we don't want use `&` in match pattern,
        // we can dereference the variable first.
        match *reference {
            val=> {
                println!("val:{}", val);  // 4
            }
        }
    }

    {
        // not like `let reference = &4;` above,
        // `_not_reference` is not a reference type, because the right side of
        // assigment `4`, is not a reference type.
        let _not_reference = 4;

        // to make the variable is a reference type, even though the right side of
        // assignment `4` is not a reference type.
        // Use the `ref` key word.
        let ref reference_now = 4;
        let ref mut mut_reference = 4;
        *mut_reference = 5;

        // 4, 5
        println!("{}, {}", *reference_now, *mut_reference);

        {
            let val1 = 1;
            match val1 {
                ref v => {
                    // type of `v` is `&i32`
                    println!("{}, {}", v, *v);  // 1, 1
                }
            }
            match val1 {
                v => {
                    // type of `v` is `i32`
                    println!("{}", v)
                }
            }

            let mut val2 = 1;
            match val2 {
                ref mut v => {
                    // type of `v` is `&mut i32`
                    *v += 10;
                }
            }
            println!("val2:{}", val2);  // 11
        }
    }

    destructure_struct_instance();
}


fn destructure_struct_instance() {
    #[derive(Debug)]
    struct Foo {
        x: (i32, i32),
        y: i32,
    }

    // destructure struct fields
    {
        let f1 = Foo { x:(1, 2), y: 3 };
        let Foo {x:(a, b), y} = f1;
        println!("{}, {}, {}", a, b, y);  // 1, 2, 3
    }

    // destructure struct fields ignore the order, and rename field name.
    {
        let f = Foo { x: (1, 2), y: 3 };
        let Foo {y: m, x: n} = f;

        println!("{:?}, {}", n, m);  // (1, 2), 3
    }

    // ignore some fields
    {
        let f = Foo { x: (1, 2), y: 3 };
        let Foo {y, ..} = f;
        println!("y:{}", y);  // 3

        let Foo {x, ..} = f;
        println!("x:{:?}", x);  // (1, 2)
    }
}