use cable_rust::serial::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }
    let port_str = &args[1];
    let port_cstring = std::ffi::CString::new(port_str.as_str()).unwrap();
    let port = port_cstring.as_ptr();
    let fd = unsafe { open(port, O_RDONLY) };

    if fd < 0 {
        eprintln!("Error: Could not open {}", port_str);
        std::process::exit(1);
    }

    loop {
        let mut buffer = [0u8; 4096];
        let bytes_read = unsafe { read(fd, buffer.as_mut_ptr(), buffer.len()) };

        if bytes_read > 0 {
            let bytes_read = bytes_read as usize;
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
        } else if bytes_read < 0 {
            eprintln!("Read error");
            break;
        }
    }

    unsafe { close(fd) };
}
