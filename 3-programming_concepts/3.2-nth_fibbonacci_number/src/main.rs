use std::io;

fn main(){

    println!("Please enter the number!");
    
    let mut input = String::new();
    let num:u32;
    
    io::stdin().read_line(&mut input).expect("Invalid input!");

    num = match input.trim().parse(){
        Ok(num)=> num,
        Err(_) => {
           println!("Please enter a valid number!");
           return;
        }
    };

    let n = fibonnaci(num);
    
    println!("The nth term is {}", n);

}

fn fibonnaci(x:u32)-> u32{
    if x <= 0{
        return 0;
    } else if x==1{
        return 1;
    } else{
        return fibonnaci(x-1)+ fibonnaci(x-2);
    }
}