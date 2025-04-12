

pub fn object_alloc<T>() -> *mut T {
    let layout = alloc::Layout::new::<T>();
    unsafe {
        alloc::alloc(layout) as *mut T
    }
}

fn object_dealloc<T>(ptr: *mut T) {
    let layout = alloc::Layout::new::<T>();
    unsafe {
        alloc::dealloc(ptr as *mut u8, layout)
    }
}

unsafe fn object_init<T: Clone>(ptr: *mut T, init_value: &T) {
    ptr::write(ptr, init_value.clone());
}

unsafe fn object_deinit<T>(ptr: *mut T) {
    ptr::drop_in_place(ptr);
}

fn main() {
    let d: Data = Data { x1: 1, x2: 2, x3: 3};
    let p: *mut Data = object_alloc();
    object_init(p, d);

    unsafe {
        (*p).x1 = 10;
    }

    object_deinit(p);
    object_dealloc(p);
}
