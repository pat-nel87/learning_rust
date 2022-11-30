pub fn add(left: usize, right: usize) -> usize {
    left + right
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
    fn another_test() {
        panic!("Arbitrary panic test!");
    }

    #[test]
    fn say_hello() {
        println!("Hello");
    }

    #[test]
    fn custom_message() {
        let result = add(2, 2);
        assert!(
            result == 5,
            "Result is not equal to 5, value was {}",
            result
        );
    }
}
