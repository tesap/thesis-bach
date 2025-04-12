use libc;
use libc::{c_int, c_char, O_RDWR, O_CREAT, O_TRUNC, close, dup};
use std::ffi::CString;

fn open(path: &str) -> i32 {
    let path_cstr = CString::new(path).unwrap();
    unsafe { libc::open(path_cstr.as_ptr(), O_RDWR | O_CREAT | O_TRUNC, 0o644) }
}

pub struct File {
    fd: i32,
}

impl File {
    // Constructor
    pub fn new(filename: &str) -> Self {
        let fd = open(filename);
        Self { fd }
    }
}

// Implementation of `Clone` trait
impl Clone for File {
    fn clone(&self) -> Self {
        let fd_dup: i32 = unsafe { libc::dup(self.fd) };
        Self { fd: fd_dup }
    }
}

impl Drop for File {
    // Destructor
    fn drop(&mut self) {
        if self.fd > 0 {
            unsafe {
                libc::close(self.fd);
            }
            self.fd = -1;
        }
    }
}

fn main() {
    let mut f1 = File::new("file1.txt");

    let f_copy = f1.clone(); // Copy emulated
    let f_move = f1; // Implicit move

    // Use moved object
    println!("{}", f_move.fd);
}
