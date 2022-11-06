fn main() {
    // The String type differ from string literals
    // string literals are immutable.
    // String type manages data on the heap
    // for when an amount of text is unknown to us at compile time
    let msg = "Appending this message";
    string_type(msg);

    // examples of different ways to interact with data
    // via moves or clone (shallow or deep copy)
    allocate_on_heap_();

    // pass by reference
    let mut new_string = String::from("Hello");
    let length = calculate_length(&new_string);
    println!("usize is : {length}, {new_string}");
    // mutable reference
    mutable_reference(&mut new_string);
    println!("{new_string}");

    // The Slice Type
    let byte_slice = first_word(&new_string);
    println!("{byte_slice}");
}
// Ownership rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn string_type(message: &str ) {
    let mut s = String::from("Hey There Neighbor!");
    s.push_str(" ");
    s.push_str(message);
    println!("{s}");
}

fn allocate_on_heap_() {
    let x = 5;
    let y = x;
    
    // When we assign s1 to s2, the String data is copied.
    // Meaning We copy the pointer, the length, and the capacity that are on the stack
    // We do not copy the data on the heap that the pointer refers to!
    let s1 = String::from("heya!");
    // If either go out of scope, Rust calls the drop function and cleans up heap memory
    // which would cause a double free error as they both point to the same memory location
    // in the heap
    let s2 = s1;
    // Rust solves this issue by making in this example, s1 invalid after
    // being assigned to s2.
    // You recieve an error if you try to use the invalidated reference, s1 after line 32
    // In Rust, we say s1 was moved into s2.
    // Rust never automatically creates "deep" copies of data.

    // if you do want to deep copy data - copy data, not just pointer and reference
    let s3 = s2.clone();
    //So we can reference both in this case
    // because the heap data is copied
    print!("s2 = {}, s3 = {}", s2, s3);

    // What about x and y?
    println!("x = {} y = {}", x, y);
    // integers have a known size at compile time & are entirely stored on the stack
    // Rust has a special annotation called "Copy" we can place on types that are stored
    // on the stack.
    // Variables that use this method do not "move" but are trivially copied
    // making assignment valid after assignment to another variable.

}

// the & indicates the function has a reference to the object taken as a parameter
// instead of taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_reference(some_string: &mut String) {
    some_string.push_str(", Wheeee!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}