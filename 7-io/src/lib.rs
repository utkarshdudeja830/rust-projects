use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Configs{
    pub term: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Configs{    
    
    pub fn new(args: &[String])-> Result<Configs, &'static str>{
        if args.len() < 3 {
            panic!("not enough arguments");
        }   
        let term = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = if args.len() > 3 {
            match args[3].to_lowercase().as_str() {
                "true" => true,
                "false" => false,
                _ => return Err("case-sensitive must be 'true' or 'false'"),
            }
        } else {
            true // Default to case-sensitive
        };
        Ok(Configs { term, filename, case_sensitive })
    }
}

pub fn run(config: Configs)-> Result<(), Box<dyn Error>> {

    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    
    let results = if config.case_sensitive {
        search(&config.term, &contents)
    } else {
        search_case_insensitive(&config.term, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(term: &str, contents:&'a str) -> Vec<&'a str>{

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(term){
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(term: &str, contents: &'a str) -> Vec<&'a str> {
    let term = term.to_lowercase(); let mut results = Vec::new();
    for line in contents.lines() {
    if line.to_lowercase().contains(&term) {
                results.push(line);
            }
    }
    results 

}