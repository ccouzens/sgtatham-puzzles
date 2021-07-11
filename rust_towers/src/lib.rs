// #[no_mangle]
// pub unsafe extern "C" fn bin2hex(input: *const u8, input_length: usize) {

// }

use libc::{c_void, malloc, size_t};

#[no_mangle]
pub unsafe extern "C" fn smalloc(size: size_t) -> *mut c_void {
    malloc(size)
}
