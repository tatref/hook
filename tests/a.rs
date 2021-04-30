use libc::{c_char, c_int, size_t};

use hook::hook;




pub unsafe fn fake_puts(real: unsafe extern fn(*const c_char) -> c_int, arg0: *const c_char) -> c_int {
    real(arg0)
}
hook!(puts, fake_puts, fn(*const c_char) -> c_int);


pub unsafe fn fake_strlen(real: unsafe extern fn(*const c_char) -> size_t, arg0: *const c_char) -> size_t {
    real(arg0)
}
hook!(strlen, fake_strlen, fn(*const c_char) -> size_t);


pub unsafe fn fake_strcmp(real: unsafe extern fn(*const c_char, *const c_char) -> c_int, arg0: *const c_char, arg1: *const c_char) -> c_int {
    real(arg0, arg1)
}
hook!(strcmp, fake_strcmp, fn(*const c_char, *const c_char) -> c_int);



