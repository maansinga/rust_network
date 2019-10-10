use std::net::TcpStream;    // We need this to access the socket file descriptor
use std::io::Write;         // To write to the socket
use std::io::Read;          // To read from the socker
use std::env;               // To get the commandline 

/// Client part of a Simple TCP/IP Echo - ClientServer program
/// 
/// This program takens commandline argument - a string and sends a Hello message to a server
/// located at 127.0.0.1:8000. The response of the server is an echo. Upon receiving this
/// echo, the application prints the response and then terminates
fn main() {
    // Getting the commandline arguments
    let args: Vec<String> = env::args().collect();

    // 1st argument shall be the client ID
    let client_id = &args[1];

    // Connecting to server
    let mut server = TcpStream::connect("127.0.0.1:8000")
        .expect("Failure in making connection to server!");

    // Building a message from temaplate
    let message: String = format!("Hello from client_{}!", client_id);

    // Writing to the server stream and verifying the result
    match server.write(message.as_bytes()) {
        Ok(_) => {
            // If all goes well, we receive Ok(usize)

            // Preparing buffer to read the server's response
            let mut buffer = [0u8; 1024];
            // Reading from the server stream
            server.read(&mut buffer).expect("Error reading from Server!");

            // Converting u8 slice to vector to string - 
            // TODO: There has to be a better way than this multiple conversion approach
            let message = String::from_utf8(buffer.to_vec()).unwrap();

            // Printing 
            println!("Response from server: {}", message);
        },
        Err(e) => {
            println!("Error writing message to server: {}", e);
        }
    }
}
