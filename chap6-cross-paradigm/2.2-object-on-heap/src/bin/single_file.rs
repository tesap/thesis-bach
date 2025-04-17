use object_as_resource_owner::File;
use object_on_heap::{object_alloc, object_init_move, object_deinit, object_dealloc};

fn main() {
    let ptr: *mut File = object_alloc();
    unsafe {
        // object_init_clone(ptr, &File::new("file2.txt"));
        object_init_move(ptr, File::new("file2.txt"));
    }

    unsafe {
        println!("FD: {}", (*ptr).fd);
    }

    unsafe {
        object_deinit(ptr);
        object_dealloc(ptr);
    }
}

