use std::thread;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::Read;
use std::io::Write;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0u8; 1024];

    let result = stream.read(&mut buffer);
    match result {
        Ok(size) => {
            if size == 0 {
                return;
            }else{
                match String::from_utf8(buffer.to_vec()){
                    Ok(message) => {
                        println!("Received From: {}, Message: {}", stream.peer_addr().unwrap(), message);
                        stream
                            .write(&buffer)
                            .expect("Failure in echo message");
                    }
                    Err(e) => println!("Connection Handler Error: {}", e)
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failure in establishing a socket binding!");

    let mut counter: i32 = 0;
    for incoming_connection in listener.incoming() {
        counter += 1;
        if counter > 10 {
            return;
        }
        match incoming_connection {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream)
                });
            },
            Err(e) => {
                println!("Error occured while accepting a connection: \n{}", e);
            }
        }
    }
    println!("Hello, world!");
}
