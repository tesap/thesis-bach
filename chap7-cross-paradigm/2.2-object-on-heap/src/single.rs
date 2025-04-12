use std::alloc;
use std::ptr;



pub fn object_alloc<T>() -> *mut T {
    let layout = alloc::Layout::new::<T>();
    unsafe {
        alloc::alloc(layout) as *mut T
    }
}

pub fn object_dealloc<T>(ptr: *mut T) {
    let layout = alloc::Layout::new::<T>();
    unsafe {
        alloc::dealloc(ptr as *mut u8, layout)
    }
}

pub unsafe fn object_init<T: Clone>(ptr: *mut T, init_value: &T) {
    ptr::write(ptr, init_value.clone());
}

pub unsafe fn object_deinit<T>(ptr: *mut T) {
    ptr::drop_in_place(ptr);
}

