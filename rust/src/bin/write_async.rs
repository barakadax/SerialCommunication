use cable_rust::serial::*;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }
    let port_str = &args[1];
    let port_cstring = std::ffi::CString::new(port_str.as_str()).unwrap();
    let port_name = port_cstring.as_ptr();
    let fd = unsafe { open(port_name, O_WRONLY | O_NOCTTY | O_NONBLOCK) };

    if fd < 0 {
        eprintln!("Error opening serial port: {}", port_str);
        std::process::exit(1);
    }

    configure_serial(fd);

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("Enter a message to send: ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut data_to_write = String::new();
        if reader.read_line(&mut data_to_write).is_err() || data_to_write.is_empty() {
            break;
        }

        let data_to_write = data_to_write.trim_end();
        if data_to_write.is_empty() {
            continue;
        }

        let mut utf16_data: Vec<u16> = Vec::new();
        utf16_data.push(0xFEFF);
        utf16_data.extend(data_to_write.encode_utf16());
        let bytes_to_write = utf16_data.len() * 2;
        
        let bytes_written = unsafe {
            write(fd, utf16_data.as_ptr() as *const u8, bytes_to_write)
        };

        if bytes_written == -1 {
            let err = get_errno();
            if err == EAGAIN || err == EWOULDBLOCK {
                eprintln!("Write buffer full, try again later.");
            } else {
                eprintln!("Error writing to serial port: {}", err);
            }
        }
    }

    unsafe { close(fd) };
}
