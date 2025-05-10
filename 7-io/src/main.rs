use std::env;

struct Configs{
    term: String,
    filename: String,
}

impl Configs{    
    
    fn new(args: &[String])-> Configs{
        if args.len() < 3 {
            panic!("not enough arguments");
        }   
        let term = args[1].clone();
        let filename = args[2].clone();
        Configs{term, filename}
    }
}


fn main(){
    let args: Vec<String> = env::args().collect();
    let configs = Configs::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguements {}", err);
        process::exit(1);
    });
    
    println!("The file to search in {} \n the term to search for in {}", configs.filename, configs.term);
}
