use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use std::env;


pub fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }
    results
}

pub fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let search = search.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&search) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let search = args[1].clone();
        let filename = args[2].clone();

        let mut case_sensitive = true;
        for (name,_) in env::vars() {
            if name == "CASE_INSENSITIVE" {
                case_sensitive = false;
                break;
            }
        }

        Ok(Config {
            search: search, 
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    
    let f_grep = if config.case_sensitive { grep } else { grep_case_insensitive };

    for line in f_grep(&config.search, &contents){
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use grep;
    use grep_case_insensitive;

    #[test]
    fn one_result() {
        let search = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["safe, fast, productive."], grep(search, contents));
    }

    #[test]
    fn case_insensitive() {
        let search = "rust";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        
        assert_eq!(vec!["Rust:", "Trust me."], grep_case_insensitive(search, contents));
    }

}
