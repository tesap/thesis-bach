use std::alloc;
use std::ptr;


fn object_alloc<T>() -> *mut T {
    let layout = alloc::Layout::new::<T>();
    unsafe {
        alloc::alloc(layout) as *mut T
    }
}

//pub unsafe fn object_init_clone<T: Clone>(ptr: *mut T, init_value: &T) {
//    ptr::write(ptr, init_value.clone());
//}

unsafe fn object_init<T>(ptr: *mut T, init_value: T) {
    ptr::write(ptr, init_value);
}


pub unsafe fn object_alloc_init<T>(init_value: T) -> *mut T {
    let ptr: *mut T = object_alloc();
    object_init(ptr, init_value); // Move
    ptr
}

fn object_dealloc<T>(ptr: *mut T) {
    let layout = alloc::Layout::new::<T>();
    unsafe {
        alloc::dealloc(ptr as *mut u8, layout)
    }
}

unsafe fn object_deinit<T>(ptr: *mut T) {
    ptr::drop_in_place(ptr);
}

pub unsafe fn object_dealloc_deinit<T>(ptr: *mut T) {
    object_dealloc(ptr);
    object_deinit(ptr);
}
