mod mem_array;
mod my_box;

use mem_array::MemArray;
use my_box::MyBox;

use std::hint::black_box;

fn main() {
    let mut m = MemArray::alloc(100);
    m.memset(111);

    let mut b = MyBox::new(m);
    black_box(&b);

    // Cannot use 'm' twice because it is moved
    // Error: Value used after move
    // b.set(m);

    let mut m2 = MemArray::alloc(200);
    m2.memset(222);

    // Can be copied multiple times
    b.set_clone(&m2);
    b.set_clone(&m2);
    b.set_clone(&m2);
    black_box(&b);
    black_box(&m2);
}

