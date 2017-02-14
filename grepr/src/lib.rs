use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use std::env;


pub fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(search))
        .collect()
}

pub fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let search = search.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&search))
        .collect()
}

pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        args.next(); //filename

        let search = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get search string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get filename"),
        };

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
