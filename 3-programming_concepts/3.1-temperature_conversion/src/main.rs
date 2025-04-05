// Simple program to convert temperatures and verify learning.

/*

fn main() {
    let c: i32;
    let f: i32 = 5;
    
    c = (f-32)*5/9;
    println!("The value in Celcius is {}", c)

}

*/

// Adding the option to get input from the user.

/*

use std::io;

fn main(){
    
    let c:i32;
    let f:i32 = 5;
    //let choice:char;
    println!("Please enter the choice of conversion!");
    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Invalid input!!");
    
    let input: char = choice.trim().chars().next().unwrap_or(' ');
    if input == 'C'{
        c = (f-32)*5/9;
        println!("The temperature is {}",c);
    }else{
        println!("Some other time");
    }

}

*/

// Finally making the program fully dynamic.

use std::io;

fn main(){
    
    println!("Please enter the converstion of your choice! Type 'C' for F->C or 'F' for C->F");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input!!");
    
    let input: char = choice.trim().chars().next().unwrap_or(' ');

    println!("Please enter the temperature!");

    let mut temp= String::new();

    io::stdin().read_line(&mut temp).expect("Invalid Temperature!!");

    let temp:f64 = match temp.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter the correct temperature!");
            return;
        }

    };

    match input {
        'C' | 'c' => {
            let c:f64;
            c = (temp -32.0)*5.0/9.0;
            println!("The temperature in celcius is {:.3}",c);
        }
        'F' | 'f' => {
            let f:f64;
            f = (9.0/5.0)*temp+32.0;
            println!("The temperature in farenheit is {:.3}",f);
        }
        _ =>{
            println!("Please enter C or F");
            
        }
    }
}