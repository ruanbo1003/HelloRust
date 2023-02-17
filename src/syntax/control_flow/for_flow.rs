
pub fn tests() {
    // visit collection by for
    {
        let a1 = [1, 2, 3, 4, 5];
        for element in a1 {
            println!("the value is: {element}");  // 1, 2, 3, 4, 5
        }

        for number in 1..4 {
            println!("the number-1 is {number}");  // 1, 2, 3
        }
        for number in (1..4).rev() {
            println!("the number-2 is {number}");  // 3, 2, 1
        }
    }

    // for number range
    {
        let mut sum = 0;
        for n in 1..101 {
            sum += n;
        }
        println!("sum:{}", sum);
    }

    // for with iterator
    {
        {
            let names = vec!["Tom", "Jack", "John"];
            for each in names.iter() {
                print!("{} ", *each);  // Tom Jack John
            }
            println!();
        }
        
    }
}