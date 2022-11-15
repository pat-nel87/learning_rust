fn main() {
    //cause_panic();
    error_recovery();
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