use std::net::TcpListener;
use std::io::Read;

const BUFF:usize = 1024;

fn main(){
    let listener = TcpListener::bind("localhost:5000").expect("Failed to bind to localhost:5000");
    println!("Escuchando en localhost:5000");

    let mut buff: [u8;BUFF] = [0;BUFF];

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let client_address = stream.peer_addr().unwrap();
                println!("Accepted connection from {}:{}",client_address.ip(), client_address.port());
                
                stream.read(&mut buff).unwrap();
            
                let readable_data = String::from_utf8(buff.to_vec()).unwrap();
                println!("The client send: {}",readable_data);
            }
            Err(e) => {
                eprintln!("Failed to accept a connection: {}",e);
            }
        }
    }
}