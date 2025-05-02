use std::ptr;

use object_on_heap::{object_alloc_init, object_dealloc_deinit};


#[derive(Debug)]
pub struct MyBox<T> {
    ptr: *mut T
}


impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        Self {
            ptr: object_alloc_init(x)
        }
    }

    pub fn from_raw(ptr: *mut T) -> Self {
        Self { ptr }
    }

    // Consumes Box, restricting its further use
    pub fn into_raw(self) -> *mut T {
        let self_ = std::mem::ManuallyDrop::new(self);
        self_.ptr
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl<T: Clone> Clone for MyBox<T> {
    fn clone(&self) -> Self {
        println!("MyBox: CLONE");
        // Without & it is treated as move, so prohibited
        // let val: &T = &**self;
        // Self::new(val.clone())
        Self::new((**self).clone())
    }

    fn clone_from(&mut self, source: &Self) {
        println!("MyBox: CLONE_FROM");

        unsafe {
            *self.ptr = (*source.ptr).clone();
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // SAFETY: ptr contains allocated value
        unsafe {
            object_dealloc_deinit(self.ptr);
        }
    }
}


