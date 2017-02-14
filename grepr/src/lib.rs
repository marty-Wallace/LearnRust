use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }
    results
}

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

    for line in grep(&config.search, &contents){
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use grep;

    #[test]
    fn one_result() {
        let search = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
    
        assert_eq!(vec!["safe, fast, productive."], grep(search, contents));
    }

}
