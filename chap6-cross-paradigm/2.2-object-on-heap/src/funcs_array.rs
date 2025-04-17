use std::alloc;
use std::ptr;

pub fn array_alloc<T>(size: usize) -> *mut T {
    let layout = alloc::Layout::array::<T>(size).unwrap();
    unsafe {
        alloc::alloc(layout) as *mut T
    }
}

pub unsafe fn array_dealloc<T>(ptr: *mut T, size: usize) {
    let layout = alloc::Layout::array::<T>(size).unwrap();
    alloc::dealloc(
        ptr as *mut u8,
        layout
    )
}

// Initialize each element in the allocated memory
//pub unsafe fn array_init_clone<T: Clone>(ptr: *mut T, size: usize, init_value: &T) {
//    for i in 0..size {
//        ptr::write(ptr.add(i), init_value.clone());
//    }
//}

pub unsafe fn array_init_move<T>(ptr: *mut T, index: usize, init_value: T) {
    ptr::write(ptr.add(index), init_value); // Enforces Move
}

pub unsafe fn array_deinit<T>(ptr: *mut T, size: usize) {
    for i in 0..size {
        ptr::drop_in_place(ptr.add(i) as *mut T);
    }
}
