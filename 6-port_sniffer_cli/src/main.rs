use std::env;
use std::net::TcpStream
const MAX: u16 = 65535;

fn main(){
    let args Vec<String> = env::args().collect();
    let mut v = Vec::new();

    let ip_address = args[0].clone();
    for port in 0..MAX{

        let mut stream = TcpStream::connect("{ip_address}:{port}")?;

        stream.write(&[1])?;
        stream.read(&mut [0; 128])?;
        Ok(())
    }
}