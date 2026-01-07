use cable_rust::serial::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }
    let port_str = &args[1];

    configure_serial(port_str);

    let mut file = match File::open(port_str) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open {}: {}", port_str, e);
            std::process::exit(1);
        }
    };

    loop {
        let mut buffer = [0u8; 4096];
        match file.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let utf8_str = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("data received: {}", utf8_str);
            }
            Ok(_) => {
                // EOF or no data
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}
