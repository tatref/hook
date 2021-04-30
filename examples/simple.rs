//#![allow(unused_variables)]
//#![allow(unused_imports)]
//#![allow(dead_code)]

use libc::{c_char, size_t};
use std::ffi::CStr;
use std::slice;

use hook::hook;





// The function strlen returns the length of a c string
// It's signature is `size_t strlen(const char *s);`
// Converted to rust: `fn(*const c_char) -> size_t`
pub unsafe fn fake_strlen(real: unsafe extern fn(*const c_char) -> size_t, arg0: *const c_char) -> size_t {
    // Will compile, but not work because calls `sys::strlen(ptr)` internally (infinite recursion)
    // https://github.com/rust-lang/rust/blob/7750402c5eaf9ed0a73cb34c8483df245c36ac7b/library/std/src/ffi/c_str.rs#L1172
    //let x = CStr::from_ptr(arg0 as *mut c_char);

    // Instead we compute the length manually
    let mut len = 0;
    let mut ptr = arg0;
    while *ptr != '\0' as i8 {
        ptr = ptr.offset(1);
        len += 1;
    }

    let the_string = CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(arg0 as *const u8, len as usize + 1));
    println!("We got: {:?}", the_string);

    // we can call the real `strlen`, or just return our own computed length
    real(arg0)
}


hook!(strlen, fake_strlen, fn(*const c_char) -> size_t);


