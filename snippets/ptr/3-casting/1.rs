use std::hint::black_box;

fn main() {
    // Reinterpret
    let mut number: i32 = 42;
    let ptr: *mut i32 = &raw mut number;

    let ptr_char: *mut char = ptr as *mut char;
    let ptr_long: *mut i64 = ptr as *mut i64;

    // Const cast
    let constant: i32 = 100;
    let ptr_const = &raw const constant as *const i32;
    let ptr_mut = ptr_const as *mut i32;
    unsafe {
        *ptr_mut = 200;
        assert_eq!(constant, 200);
    }


    // Type coercion (safe, automatic)
    let s = "Hello, Rust!";
    let t: &str = s; // Coercion from &String to &str
    println!("Type coercion: {}", t);
}
