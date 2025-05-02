
mod funcs_single;
mod funcs_array;

pub use funcs_single::{object_alloc_init, object_dealloc_deinit};
pub use funcs_array::{array_alloc, array_init, array_dealloc, array_deinit};
