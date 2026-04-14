use std::io::Read;
use std::net::TcpListener;

fn main() {
    // Bind to all interfaces on port 8080
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Could not bind");
    println!("RPi Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let mut buffer = [0; 128];
                match s.read(&mut buffer) {
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buffer[..n]);
                        println!("ESP32 says: {}", msg);
                    }
                    Err(e) => println!("Failed to read: {}", e),
                }
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}