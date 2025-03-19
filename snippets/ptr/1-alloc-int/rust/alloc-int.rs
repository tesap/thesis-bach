use std::hint::black_box;

fn main() { unsafe {
    let ptr: *mut i32 = &mut 111111;
    black_box(ptr);
    black_box(*ptr);

    *ptr = 222222;
    black_box(ptr);
} }
