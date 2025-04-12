mod single;

use memory_management::Data;
use single::{object_alloc, object_init, object_deinit, object_dealloc};


fn main() {
    let d: Data = Data { x1: 1, x2: 2, x3: 3};
    let p: *mut Data = object_alloc();
    unsafe {
        object_init(p, &d);
    }

    unsafe {
        (*p).x1 = 10;
    }

    unsafe {
        object_deinit(p);
        object_dealloc(p);
    }
}

