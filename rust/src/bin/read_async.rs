use cable_rust::serial::*;
use std::fs::OpenOptions;
use std::io::{Read, ErrorKind};
use std::os::unix::fs::OpenOptionsExt;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }
    let port_str = &args[1];

    configure_serial(port_str);

    let mut file = match OpenOptions::new()
        .read(true)
        .custom_flags(O_NONBLOCK)
        .open(port_str)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open {}: {}", port_str, e);
            std::process::exit(1);
        }
    };

    loop {
        thread::sleep(Duration::from_millis(300));

        let mut buffer = [0u8; 4096];
        match file.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let u16_count = bytes_read / 2;
                let mut u16_data = Vec::with_capacity(u16_count);

                for i in (0..bytes_read & !1).step_by(2) {
                    let code_unit = (buffer[i] as u16) | ((buffer[i + 1] as u16) << 8);
                    u16_data.push(code_unit);
                }

                if !u16_data.is_empty() {
                    match String::from_utf16(&u16_data) {
                        Ok(mut utf8_str) => {
                            if utf8_str.starts_with('\u{feff}') {
                                utf8_str.remove(0);
                            }
                            println!("data received: {}", utf8_str);
                        }
                        Err(_) => {
                            eprintln!("UTF conversion error");
                        }
                    }
                }
            }
            Ok(_) => {
                // No data available (EOF is unlikely on serial port unless disconnected)
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                // Expected in non-blocking mode
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}
