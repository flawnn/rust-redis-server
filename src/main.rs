use std::{net::{TcpListener, TcpStream}, thread, io::{Read, Write}};


fn main() {
    println!("Server started up! 💯");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");

                thread::spawn(move || {
                    handle_client(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received message: {}", message);
                
                if message.contains("ping"){
                    let _ = stream.write_all(b"+PONG\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}
