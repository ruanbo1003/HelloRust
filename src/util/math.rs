
pub fn my_add<T>(a: T, b: T) -> T
    where T: std::ops::Add<T, Output=T>
{
    a + b
}

pub fn is_equal<T: std::cmp::PartialEq>(a: T, b: T) -> bool {
    a == b
}

/*
unit tests.
create a tests guess_game in each guess_game you want test / in each source file you want test.
`use super::*` to access all ancestor's function.
 */
mod tests {
    #![allow(unused_imports)]
    use crate::util::math::{is_equal, my_add};

    #[test]
    fn add_test() {
        // add
        let a = 10;
        let b = 10;
        assert_eq!(my_add(a, b), a+b);
    }

    #[test]
    fn equal_test() {
        // equal
        {
            let a1 = 1;
            let b1 = 1;
            assert!(is_equal(a1, b1));

            let a2 = true;
            let b2 = true;
            assert!(is_equal(a2, b2));

            let a3 = 10.1;
            let b3 = 10.1;
            assert!(is_equal(a3, b3));
        }

        // not equal
        {
            let a1 = 1;
            let a2 = 2;
            assert!(!is_equal(a1, a2));
        }
    }

}

