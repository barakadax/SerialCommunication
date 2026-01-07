use cable_rust::serial::*;
use std::io::{self, BufRead, Write};
use std::fs::OpenOptions;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }
    let port_str = &args[1];

    configure_serial(port_str);

    let mut file = match OpenOptions::new()
        .write(true)
        .open(port_str) 
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening serial port: {}: {}", port_str, e);
            std::process::exit(1);
        }
    };

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("Enter a message to send: ");
        io::stdout().flush().unwrap();

        let mut data_to_write = String::new();
        if reader.read_line(&mut data_to_write).is_err() || data_to_write.is_empty() {
            break;
        }

        let data_to_write = data_to_write.trim_end();
        if data_to_write.is_empty() {
            continue;
        }

        if let Err(e) = file.write_all(data_to_write.as_bytes()) {
            eprintln!("Error writing to serial port: {}", e);
        }
    }
}
