use object_as_resource_owner::File;
use object_on_heap::{object_alloc_init, object_dealloc_deinit};

fn main() {
    // let ptr: *mut File = object_alloc();
    // unsafe {
    //     let f = File::new("file2.txt");
    //     object_init(ptr, f);
    // }

    let f = File::new("file2.txt");
    unsafe {
        let ptr: *mut File;
        ptr = object_alloc_init(f);
        println!("FD: {}", (*ptr).fd);
        object_dealloc_deinit(ptr);
    }
}

