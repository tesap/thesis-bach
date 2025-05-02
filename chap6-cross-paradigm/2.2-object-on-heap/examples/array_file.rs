use object_as_resource_owner::File;
use object_on_heap::{array_init, array_alloc, array_deinit, array_dealloc};

fn main() {
    println!("ARRAY MAIN");
    let size: usize = 10;
    let ptr: *mut File = array_alloc(size);

    unsafe {
        array_init(ptr, 0, File::new("file1.txt"));
        array_init(ptr, 1, File::new("file2.txt"));
        array_init(ptr, 2, File::new("file3.txt"));
    }


    unsafe {
        // (*p.add(5)).x1 = 10;
        println!("FD: {}", (*ptr).fd);
        println!("FD: {}", (*ptr.add(1)).fd);
        println!("FD: {}", (*ptr.add(2)).fd);
    }

    unsafe {
        array_deinit(ptr, 3);
        array_dealloc(ptr, size);
    }
}

