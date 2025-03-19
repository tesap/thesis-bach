use std::hint::black_box;

fn main() {
    let mut n: i32 = 111111;
    let ptr: *mut i32 = &mut n;
    black_box(ptr);

    unsafe {
        *ptr = 222222;
        black_box(ptr);
    }
}
