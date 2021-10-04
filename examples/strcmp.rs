use libc::*;
use std::ffi::CStr;

use hook::hook;

pub unsafe fn fake_strcmp(
    _real: unsafe extern "C" fn(*const c_char, *const c_char) -> c_int,
    arg0: *const c_char,
    arg1: *const c_char,
) -> c_int {
    let _a = CStr::from_ptr(arg0 as *mut c_char);
    let _b = CStr::from_ptr(arg1 as *mut c_char);

    // we can call the real `strcmp`, or just return our own computed length
    //_real(arg0, arg1)
    0
}

hook!(
    strcmp,
    fake_strcmp,
    fn(*const c_char, *const c_char) -> c_int
);
