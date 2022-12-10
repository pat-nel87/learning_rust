use std::process;

fn main() {
    
   if let Err(e) = bmp_builder::run() {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
