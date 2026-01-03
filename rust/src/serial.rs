use std::process::Command;

pub const O_NONBLOCK: i32 = 0x800;

pub fn configure_serial(port: &str) {
    let status = Command::new("stty")
        .arg("-F")
        .arg(port)
        .arg("115200")
        .arg("cs8")
        .arg("-parenb")
        .arg("-cstopb")
        .arg("-crtscts")
        .arg("raw")
        .arg("min")
        .arg("1")
        .arg("time")
        .arg("0")
        .status();

    if let Err(e) = status {
        eprintln!("Failed to execute stty: {}", e);
    } else if let Ok(s) = status {
        if !s.success() {
            eprintln!("stty exited with error");
        }
    }
}
