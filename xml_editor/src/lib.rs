use std::env;
use std::error::Error;
use std::fs;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub struct Config<'a> {
    pub file_path: &'a String,
}

impl Config<'_> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("1 Minimum arguments required");
        }

        let file_path = &args[1];

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    //println!("Filepath is {contents}");
    read_xml(&contents);
    Ok(())
}

pub fn read_xml(xml_file: &str) {
    let mut reader = Reader::from_str(&xml_file);
    reader.trim_text(true);
    
    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
       match reader.read_event_into(&mut buf) {
        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        // exits the loop when reaching end of file
        Ok(Event::Eof) => break,

        Ok(Event::Start(e)) => {
            match e.name().as_ref() {
                b"book" => println!("attributes values: {:?}",
                                    e.attributes().map(|a| a.unwrap().value)
                                    .collect::<Vec<_>>()),
                b"author" => count += 1,
                _ => (),
            }
        }
        Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

        // There are several other `Event`s we do not consider here
        _ => (),
    }
    // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
    buf.clear();
    }
}
