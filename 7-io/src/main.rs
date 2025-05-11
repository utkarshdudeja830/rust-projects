use std::env;
extern crate io;
use std::process;
use io::Configs;

fn main(){
    let args: Vec<String> = env::args().collect();
    let configs = Configs::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguements {}", err);
        process::exit(1);
    });
    
    println!("The file to search in {} \n the term to search for in {}", configs.term, configs.filename);

    if let Err(e) = io::run(configs){
        println!("An error has occured: {}",e);
        process::exit(1);
    }
}