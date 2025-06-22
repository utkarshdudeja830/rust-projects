use std::env;
use std::net::TcpStream;
const MAX: u16 = 65335;

fn scan(ip_address: &str, port: u16) -> bool{
    match TcpStream::connect(format!("{}:{}", ip_address ,port)){
        Ok(_) => true,
        Err(_) => false,
    }
}
fn main(){
    let args: Vec<String> = env::args().collect();
    // let mut v = Vec::new();

    let ip_address = args[1].clone();

    for port in 0..MAX{
        if scan(&ip_address, port) {
            println!("Port {}: Open", port);
        }
    }

}