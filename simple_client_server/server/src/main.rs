use std::thread;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::Read;
use std::io::Write;

/// Handles a single connection / stream
fn handle_connection(mut stream: TcpStream) {
    // Buffer to read the client's message
    let mut buffer: [u8; 1024] = [0u8; 1024];

    // Reading from the client - client has to message first!
    let result = stream.read(&mut buffer);
    match result {
        Ok(size) => {
            if size == 0 {
                // When the cient terminates abruptly, the
                // server goes into an infinite loop(if this is in a loop) printing
                // the last message of the client. This can be
                // voided if we break upon 0 size read
                return;
            }else{
                // Reading the message
                let message = String::from_utf8(buffer.to_vec()).expect("Error reading string from buffer");
                println!("Received From: {}, Message: {}", stream.peer_addr().unwrap(), message);
                stream
                    .write(&buffer)
                    .expect("Failure in echo message");
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

/// This is Server part of a Simple TCP/IP Echo Client-Server program.
/// 
/// The server binds to 127.0.0.1:8000 and awaits incoming connections.
/// The received connection is handed off to a handler that runs in a thread.alloc
/// Once the server handles 10 requests, it terminates.
fn main() {
    // Listener for incoming connections
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failure in establishing a socket binding!");
    // Connection counter
    let mut counter: i32 = 0;

    // For each incoming connection...
    for incoming_connection in listener.incoming() {
        counter += 1;
        if counter > 10 {
            return;
        }

        // If the connection is not an error?
        match incoming_connection {
            Ok(stream) => {
                // Hand over the handling to a handle_connection() running in a thread
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
