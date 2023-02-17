
pub fn tests() {
    let mut counter = 1;
    loop {
        if counter > 3 {
            break;  // break the loop
        }
        println!("loop {counter}");
        counter += 1;
    }

    // return value from loop
    let mut counter2 = 0;
    let result2 = loop {
        counter2 += 1;
        if counter2 >= 10 {
            break counter2 * 2;  // break the loop and return a value
        }
    };
    println!("loop value result2:{result2}"); // 20

    let mut counter3 = 0;
    'outer_loop: loop {
        println!("counter3:{counter3}");

        let mut inner_counter = 0;
        loop {
            inner_counter += 1;
            println!("inner_counter:{inner_counter}");
            if inner_counter == 2 {
                break;
            }

            if counter3 == 3 {
                break 'outer_loop;  // break the specified loop
            }
        }

        counter3 += 1;
    }
    println!("nested loop, counter3:{counter3}");  // 3

}