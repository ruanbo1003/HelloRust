

#[cfg(test)]
mod tests {
    #[test]
    fn equal_test() {
        // use assert! micro
        assert!(1 == 1);
        assert!(true == true);

        // use assert_eq! micro
        assert_eq!(1, 1);
        assert_eq!(true, true);
    }

    #[test]
    fn not_equal_test() {
        // use assert! micro
        assert!(!(1 == 2));
        assert!(!(true == false));

        // use assert_ne! micro
        assert_ne!(1, 2);
        assert_ne!(true, false);
    }

    #[test]
    fn test_with_failure_message() {
        // when test failed, the message will output.
        // assert_eq!(1, 2, "message: the equal test failed");
    }

    // test with Result<T, E>
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 1 + 1 == 2 {
            Ok(())
        } else {
            Err(String::from(""))
        }
    }

    // ignored tests
    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(1, 1);
    }
}