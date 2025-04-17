use libc;
use std::ffi::CString;

fn open(path: &str) -> i32 {
    let path_cstr = CString::new(path).unwrap();
    unsafe { libc::open(path_cstr.as_ptr(), libc::O_RDWR | libc::O_CREAT | libc::O_TRUNC, 0o644) }
}

pub struct File {
    pub fd: i32,
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

