use std::os::unix::io::RawFd;

pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
pub const O_NOCTTY: i32 = 0x100;
pub const O_NONBLOCK: i32 = 0x800;

pub const B115200: u32 = 0o010002;
pub const CS8: u32 = 0x30;
pub const PARENB: u32 = 0x100;
pub const CSTOPB: u32 = 0x40;
pub const CRTSCTS: u32 = 0x80000000;
pub const CSIZE: u32 = 0x30;

pub const TCSANOW: i32 = 0;

pub const VMIN: usize = 6;
pub const VTIME: usize = 5;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

unsafe extern "C" {
    pub fn open(path: *const i8, flags: i32) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
    pub fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    pub fn tcgetattr(fd: i32, termios_p: *mut termios) -> i32;
    pub fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const termios) -> i32;
    pub fn cfsetispeed(termios_p: *mut termios, speed: u32) -> i32;
    pub fn cfsetospeed(termios_p: *mut termios, speed: u32) -> i32;
}

#[cfg(target_os = "linux")]
pub fn get_errno() -> i32 {
    unsafe {
        *__errno_location()
    }
}

unsafe extern "C" {
    fn __errno_location() -> *mut i32;
}

pub const EAGAIN: i32 = 11;
pub const EWOULDBLOCK: i32 = 11;

pub fn configure_serial(fd: RawFd) {
    unsafe {
        let mut tty: termios = std::mem::zeroed();
        if tcgetattr(fd, &mut tty) != 0 {
            eprintln!("Error from tcgetattr");
            return;
        }

        cfsetispeed(&mut tty, B115200);
        cfsetospeed(&mut tty, B115200);

        tty.c_cflag &= !PARENB;
        tty.c_cflag &= !CSTOPB;
        tty.c_cflag &= !CSIZE;
        tty.c_cflag |= CS8;
        tty.c_cflag &= !CRTSCTS;

        tty.c_cc[VMIN] = 1;
        tty.c_cc[VTIME] = 0;

        if tcsetattr(fd, TCSANOW, &tty) != 0 {
            eprintln!("Error from tcsetattr");
        }
    }
}
