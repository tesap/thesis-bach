

use std::hint::black_box;

fn mainn2(n: usize) {
    let arr: [i32; 3] = [10, 20, 30];
    let ptr: *const i32 = arr.as_ptr();
    black_box(ptr);

    // --- Addition
    unsafe {
        black_box(n);
        let ptr_add: *const i32 = ptr.add(n);
        black_box(ptr_add);
    }

    // --- Dereference
    unsafe {
        let mut i0: i32 = ptr.read();
        black_box(i0);
        // OR
        // let mut i0: i32 = *ptr;
    }

    // --- Combined: Addition + Dereference
    unsafe {
        let i2: i32 = *ptr.add(2);
        black_box(i2);
    }
}

fn main() {
    let s = String::from("2");
    let s2 = format!("{:}", s.parse::<usize>().unwrap());
    let n = s2.parse::<usize>().unwrap();
    mainn2(n);
}
