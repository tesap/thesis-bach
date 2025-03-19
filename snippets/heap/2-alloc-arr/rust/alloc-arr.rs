use std::alloc;
use std::mem;
use std::hint::black_box;

type Layout = alloc::Layout;

fn array_layout<T>(count: usize) -> Layout {
    let layout = alloc::Layout::array::<T>(count).unwrap();
    assert_ne!(layout.size(), 0);
    assert_eq!(layout.size(), count * mem::size_of::<T>());
    layout
}

fn main() {
    let layout = array_layout::<u8>(1000);

    let ptr = unsafe {
        alloc::alloc(layout) as *mut u8
    };

    black_box(ptr);

}
