extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game!");
    
    
    loop {
    let secret_number = rand::thread_rng().gen_range(1,11);
    println!("Please enter the number!");
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("Falied to read the line");
    
    let guess: u32 = match guess.trim().parse() {
        Ok(num)=>num,
        Err(_)=> continue,
    };

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too less!!"),
        Ordering::Greater => println!("Too Big!!"),
        Ordering::Equal =>{
            println!("You Win!!");
            break;
        }
    }

    println!("You guessed {}", guess);
    println!("The secret number is {}",secret_number);

    }
}
