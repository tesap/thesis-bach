use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::hint::black_box;

fn alloc_array(size: usize) -> *mut i32 {
    let layout = Layout::array::<i32>(size).unwrap();

    unsafe {
        alloc_zeroed(layout) as *mut i32
    }
}

fn dealloc_array(p: *mut i32, size: usize) {
    let layout = Layout::array::<i32>(size)
        .unwrap();

    unsafe {
        dealloc(p as *mut u8, layout)
    }
}

struct A {
    //pub p: *mut i32,
    pub x1: i64,
    pub x2: i64,
    pub x3: i64
}

impl A {
    fn new(x1: i64, x2: i64, x3: i64) -> Self {
        A {
            //p: alloc_array(1000),
            x1: x1,
            x2: x2,
            x3: x3,
        }
    }
}

//impl Drop for A {
//    fn drop(&mut self) {
//        dealloc_array(self.p, 1000);
//    }
//}

fn main() {
    let x = A::new(1, 2, 3);
    black_box(&x.x1);
    let y = x;
    black_box(&y.x1);
}
