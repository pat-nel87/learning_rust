use std::process;
use std::env;

use xml_editor::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Arguments could not be parsed! Error: {err}");
        process::exit(1);
    });

    if let Err(e) = xml_editor::run(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
