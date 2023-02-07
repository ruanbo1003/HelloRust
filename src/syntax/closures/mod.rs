#![allow(dead_code, unused_variables)]

mod closure_capture;
mod iterators;
mod my_iterator;


pub fn tests() {
    // function_and_closure();

    // closure_capture::tests();

    // iterators::tests();

    my_iterator::tests();
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    print!("calculating slowly...");
    std::thread::sleep(std::time::Duration::from_secs(1));
    return intensity;
}

fn fitness_play() {
    let user_specified_value = 10;
    let random_num = 7;
}

/*
in this function, we call the function `simulated_expensive_calculation`
three times with the same parameter `intensity`.
 */
fn generate_workout_1(intensity: u32, random_num: u32) {
    if intensity < 23 {
        println!(
            "Today, do {} push-ups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} sit-ups!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            )
        }
    }
}

fn generate_workout_2(intensity: u32, random_number: u32) {
    // here, we init a expensive_result variable as `simulated_expensive_calculation(intensity)`
    // By doing this, we do not call `simulated_expensive_calculation(intensity)` three times in out function body.
    // But we must call the function `simulated_expensive_calculation(intensity)` every time.
    // including the `if random_number == 3` branch which do not need the call.
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_result
        );
        println!(
            "Next, do {} sit-ups!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today run for {} minutes",
                expensive_result
            )
        }
    }
}


fn generate_workout_3(intensity: u32, random_number: u32) {
    //
    let closure_1 = |num: u32| {
        num
    };

    // the capture type is u32 which is specified.
    // the return type is u32, which is deducted.
    let closure_2 = |num:u32| {
        num
    };
}

fn function_and_closure() {
    // function and closure to implement add one to an u32.
    {
        fn add_one_1(x: u32) -> u32 { x + 1 }  // a function
        let add_one_2 = |x: u32| -> u32 { x + 1 };  // a full annotation closure
        let add_one_3 = |x| { x + 1 };  // a non-annotation closure, the annotation type will be deduced wen called.
        let add_one_4 = |x| x + 1;

        let u1 = add_one_1(10);
        let u2 = add_one_2(10);
        let u3 = add_one_3(10);
        let u4 = add_one_4(10);

        println!("{u1}, {u2}, {u3}, {u4}");  // 11, 11, 11, 11
    }

    // deduce annotation
    {
        let closure = |x| x;
        let s = closure(String::from("Hello"));
        println!("s:{s}");  // Hello

        let s2 = closure(String::from("Rust"));
        println!("s2:{s2}");  // Rust

        // When call closure multiple times, the parameter of each call must same.
        // If not, it's error.
        // let i = closure(1);  // Error: expected struct `String`, found integer.
    }
}


/*
member variable of ValCache are private, managed by ValCache.
 */
struct ValCache<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> ValCache<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> ValCache<T> {
        ValCache {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout_4(intensity: u32, random_number: u32) {
    let mut expensive_result = ValCache::new(|num| {
        println!("closure calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}