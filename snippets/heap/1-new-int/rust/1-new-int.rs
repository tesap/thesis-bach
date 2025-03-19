use std::hint::black_box;

use std::alloc::{Layout, alloc, dealloc};

fn main() {
    let layout = Layout::new::<i32>();
    unsafe {
        let int_ptr = alloc(layout) as *mut i32;
        black_box(&int_ptr);
        *int_ptr = 10;
        black_box(&int_ptr);
        dealloc(int_ptr as *mut u8, layout);
        black_box(&int_ptr);
    }
}
