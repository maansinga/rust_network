use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let client_id = &args[1];
    let mut server = TcpStream::connect("127.0.0.1:8000")
        .expect("Failure in making connection to server!");

    let message: String = format!("Hello from client_{}!", client_id);

    match server.write(message.as_bytes()) {
        Ok(_) => {
            let mut buffer = [0u8; 1024];
            server.read(&mut buffer).expect("Error reading from Server!");

            let message = String::from_utf8(buffer.to_vec()).unwrap();
            println!("Response from server: {}", message);
        },
        Err(e) => {
            println!("Error writing message to server: {}", e);
        }
    }
}
