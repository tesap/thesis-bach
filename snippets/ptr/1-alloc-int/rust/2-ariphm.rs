use std::hint::black_box;

fn main() {

    let arr: [i32; 3] = [10, 20, 30];
    let ptr: *const i32 = arr.as_ptr();

    black_box(ptr);

    unsafe {
        // Equivalent
        let mut i0: i32;
        i0 = *ptr;
        black_box(i0);
        i0 = ptr.read();
        black_box(i0);

        let i1: i32 = *ptr.add(1);
        black_box(i1);
        let i2: i32 = *ptr.add(2);
        black_box(i2);
    }
}
