use std::alloc;

#[derive(Debug)]
pub struct MyBox<T> {
    pub ptr: *mut T
}

impl<T: Clone> MyBox<T> {
    #[inline(never)]
    pub fn set_clone(&mut self, value: &T) {
        unsafe {
            self.ptr.write(value.clone());
        }
    }
}

impl<T> MyBox<T> {
    #[inline(never)]
    pub fn new(value: T) -> Self {
        let ptr = unsafe {
            let layout = alloc::Layout::new::<T>();
            let ptr = alloc::alloc(layout) as *mut T;
            ptr.write(value);
            ptr
        };

        Self { ptr }
    }

    #[inline(never)]
    pub fn set_move(&mut self, value: T) {
        unsafe {
            self.ptr.write(value);
        }
    }

    // Returns a bitwise copy
    #[inline(never)]
    pub fn get(&self) -> T {
        unsafe {
            self.ptr.read()
        }
    }
}

