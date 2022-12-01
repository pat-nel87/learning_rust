use std::env;
use std::process;

use minigrep::Config;
/*
Note that std::env::args will panic if any argument contains invalid Unicode.
If your program needs to accept arguments containing invalid Unicode,
use std::env::args_os instead. That function returns an iterator that produces OsString values
instead of String values. We’ve chosen to use std::env::args here for simplicity,
because OsString values differ per platform and are more complex to work with than String values.
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem with parsing the arguments!: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}





