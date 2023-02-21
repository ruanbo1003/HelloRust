
/*
When closure as parameter, we must annotation the whole type of the closure, this is implemented
by using one of the trait bellow.
    1. Fn: closure capture by `&T` (immutable reference)
    2. FnMut: closure capture by `&mut T` (mutable reference)
    3. FnOnce: closure capture by `T` (value, take ownership)
 */

pub fn tests() {
    // fn_parameter();

    // fn_once_parameter();

    closure_parameter_type_annotation();
}

#[derive(Debug)]
struct FnParam {
    data: i32,
}

fn fn_once_parameter() {
    fn apply_to_3<F>(f: F) -> i32 where
        F:FnOnce(i32) -> i32 {
        return f(3);
    }

    // Fn closure can passed as FnOnce parameter
    {
        let fp = FnParam { data: 1 };
        let double = |x: i32| {
            println!("{:?}", fp);
            return x * 2;
        };

        let r = apply_to_3(double);
        println!("r:{:?}", r);  // 6
        println!("{:?}", fp);  // FnParam { data: 1 }
    }

    // FnMut closure can passed as FnOnce parameter
    {
        let mut fp = FnParam { data: 1 };
        let double = |x: i32| {
            fp.data = 2;
            return x * 2;
        };
        let r = apply_to_3(double);
        println!("r:{}", r);  // 6
        println!("{:?}", fp);  // FnParam { data: 2 }
    }

    {
        let fp = FnParam { data: 1 };
        let double = |x: i32| {
            println!("{:?}", fp);
            std::mem::drop(fp);  // drop fp here, so this closure is FnOnce
            return x * 2;
        };
        let r = apply_to_3(double);
        println!("r:{}", r);  // 6

        // borrowed of moved value: `fp`
        // println!("{:?}", fp);  // error
    }
}

fn fn_parameter() {
    fn apply_to_3<F>(f: F) -> i32 where
        F: Fn(i32) -> i32 {
        return f(3);
    }

    // closure `double` just print the `fp`, add immutable reference(&T) is enough.
    {
        let fp = FnParam { data: 1 };
        let double = |x: i32| {
            println!("{:?}", fp);
            x * 2
        };
        let r = apply_to_3(double);
        println!("r:{}", r);
    }

    // closure `double` implemented `FnMut`, cannot pass to function apply_to_3
    {
        let mut fp = FnParam { data: 1 };
        let double = |x: i32| {
            fp.data = 2;  // closure is `FnMut` because if mutates the variable `fp.data` here.
            return x * 2;
        };

        // expected a closure that implement the `Fn` trait, but this closure only implement `FnMut`.
        // let r = apply_to_3(double);  // error
    }

}

fn closure_parameter_type_annotation() {
    // closure parameter: implement `FnOnce` trait, no parameter, no return.
    fn call_closure_1<F>(f: F) where
        F: FnOnce() {
        f();
    }

    // closure parameter: implement `FnOnce` trait, i32 parameter, no return.
    fn call_closure_2<F>(f: F) where
        F: FnOnce(i32) {
        f(1);
    }

    // closure parameter: implement `FnMut` trait, two i32 parameter, i32 and string return.
    fn call_closure_3<F>(mut f: F) where
        F: FnMut(i32, i32) -> (i32, String) {
        f(1, 2);
    }

    fn call_closure_4<F>(f: F) -> i32 where
        F: Fn(i32, i32) -> (i32, String) {
        let (i, s) = f(1, 2);
        println!("{},{}", i, s);
        return 10;
    }
}