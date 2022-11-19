use std::io;

fn main() {
    //cause_panic();
    error_recovery();
    let propagate_one = propagate_error_example().unwrap();
    println!("{:?}", propagate_one);

    let propagate_two = propagate_q_operator().unwrap();
    println!("{:?}", propagate_two);

    let simpler = simple_read(String::from("hello.txt")).expect("File doesn't exist!");
    println!("{:?}", simpler);

}

fn cause_panic() {
    panic!("This is an example of panic!");
}

fn error_recovery() {
    use std::fs::File;
    use std::io::ErrorKind;
    let file_result = File::open("hello.txt");
    
    let file_match = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    
}

fn propagate_error_example() -> Result<String, io::Error > {
    use std::fs::File;
    use std::io::{self, Read};
    
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn propagate_q_operator() -> Result<String, io::Error> {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn simple_read(filename: String) -> Result<String, io::Error> {
    use std::fs;
    fs::read_to_string(filename)
}
