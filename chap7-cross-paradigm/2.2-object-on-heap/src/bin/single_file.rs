use object_as_resource_owner::File;
use object_on_heap::{object_alloc, object_init, object_deinit, object_dealloc};

fn main() {
    let ptr: *mut File = object_alloc();
    unsafe {
        object_init(ptr, &File::new("file2.txt"));
    }

    unsafe {
        println!("FD: {}", (*ptr).fd);
    }

    unsafe {
        object_deinit(ptr);
        object_dealloc(ptr);
    }
}

