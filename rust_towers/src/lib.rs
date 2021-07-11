// #[no_mangle]
// pub unsafe extern "C" fn bin2hex(input: *const u8, input_length: usize) {

// }

use libc::{c_void, free, malloc, size_t, realloc, strcpy, strlen, c_char};

#[no_mangle]
pub unsafe extern "C" fn smalloc(size: size_t) -> *mut c_void {
    malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn sfree(p: *mut c_void) {
    if !p.is_null() {
        free(p);
    }
}

#[no_mangle]
pub unsafe extern "C" fn srealloc(p: *mut c_void, size: size_t) -> *mut c_void {
    if !p.is_null() {
        realloc(p, size)
    } else {
        malloc(size)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dupstr(s: *const c_char) -> *mut c_char {
    let r = smalloc(1 + strlen(s)) as *mut c_char;
    strcpy(r, s);
    r
}
