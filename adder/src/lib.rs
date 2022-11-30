pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn panic_function() {
    panic!("Arbitrary panic test!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // At its simplest, a test in Rust is a function that’s annotated with the test attribute
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // We might also have non-test functions in the tests module to
    // help set up common scenarios or perform common operations
    // so we always need to indicate which functions are tests using the test attribute.

    #[test]
    fn say_hello() {
        println!("Hello");
    }

    #[test]
    #[ignore]
    fn custom_message() {
        let result = add(2, 2);
        assert!(
            result == 5,
            "Result is not equal to 5, value was {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn should_panic_test() {
        panic_function();
    }

    #[test]
    #[ignore]
    fn ignore_test_example() {
        panic!("Test fails, ignore unless specifically requested")
    }

}
