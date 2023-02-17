
pub fn tests() {
    let mut num1 = 3;
    while num1 != 0 {
        println!("num1:{num1}");  // 3, 2, 1
        num1 -= 1;
    }

    let a = [1, 2, 3, 4, 5];
    let mut idx = 0;
    while idx < 5 {
        println!("the value of a[{idx}] is {}", a[idx]);  // 1, 2, 3, 4, 5
        idx += 1;
    }
}