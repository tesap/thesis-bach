use std::alloc;
use std::ptr;

pub struct MemArray
{
    pub ptr: *mut u8,
    pub length: usize,
}

impl Clone for MemArray {
    fn clone(&self) -> Self {
        let c = Self::alloc(self.length);
        unsafe {
            self.ptr.copy_to(c.ptr, self.length);
        }
        c
    }
}

impl MemArray {
    // Constructor
    #[inline(never)]
    pub fn alloc(length: usize) -> Self {
        let ptr = unsafe {
            let layout = alloc::Layout::array::<u8>(length).unwrap();
            alloc::alloc(layout) as *mut u8
        };


        Self { ptr, length }
    }

    #[inline(never)]
    pub fn memset(&mut self, value: u8) {
        unsafe {
            for i in 0..self.length {
                self.ptr.add(i).write(value);
            }
        }
    }

    #[inline(never)]
    pub fn dealloc(&mut self) {
        unsafe {
            let layout = alloc::Layout::array::<u8>(self.length).unwrap();
            alloc::dealloc(self.ptr, layout);
        }

        self.ptr = ptr::null::<u8>() as *mut u8;
        self.length = 0;
    }

    // Not needed
    //pub fn move(self) {
    //    Self {
    //        ptr: self.ptr,
    //        length: self.length
    //    }
    //}
}

impl Drop for MemArray {
    #[inline(never)]
    fn drop(&mut self) {
        self.dealloc();
    }
}

