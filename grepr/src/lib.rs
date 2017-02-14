use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub struct Config {
    pub search: String,
    pub filename: String,
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let search = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            search: search, 
            filename: filename,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);

    Ok(())
}
