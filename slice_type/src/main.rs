fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("{} ", word);

    s.clear(); // this empties the String, making it equal to ""
}

// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
// A slice is a kind of reference, so it does not have ownership.


// Improving the first_word function by using a string slice for the type of the s parameter
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    // returns a slice!
    &s[..]
}
