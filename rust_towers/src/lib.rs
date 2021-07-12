use libc::{c_char, c_int, c_uchar, c_void, free, malloc, realloc, size_t};
use std::ffi::CStr;
use std::mem::size_of;

#[no_mangle]
pub extern "C" fn smalloc(size: size_t) -> *mut c_void {
    unsafe { malloc(size) }
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
        smalloc(size)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dupstr(s: *const c_char) -> *mut c_char {
    let s = CStr::from_ptr(s).to_bytes_with_nul();
    let r = snewn::<u8>(s.len());
    s.iter().zip(r.iter_mut()).for_each(|(&a, b)| *b = a);
    r.as_mut_ptr() as *mut c_char
}

pub fn snewn<T>(number: size_t) -> &'static mut [T] {
    unsafe { std::slice::from_raw_parts_mut(smalloc(number * size_of::<T>()) as *mut T, number) }
}

#[no_mangle]
pub unsafe extern "C" fn bin2hex(input: *const c_uchar, inlen: c_int) -> *mut c_char {
    let input = std::slice::from_raw_parts::<c_uchar>(input, inlen as usize);
    let output = snewn::<c_char>((inlen * 2 + 1) as size_t);
    for (p, i) in output.iter_mut().zip(0..inlen * 2) {
        let mut v = input[i as usize / 2];
        if i % 2 == 0 {
            v >>= 4;
        };
        *p = b"0123456789abcdef"[v as usize & 0xF] as c_char;
    }
    output[inlen as usize * 2] = b'\0' as c_char;
    output.as_mut_ptr()
}
