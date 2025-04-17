
mod funcs_single;
mod funcs_array;

pub use funcs_single::{object_alloc, object_init_move, object_dealloc, object_deinit};
pub use funcs_array::{array_alloc, array_init_move, array_dealloc, array_deinit};
