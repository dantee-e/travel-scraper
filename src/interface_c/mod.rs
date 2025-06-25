pub mod city;
pub mod free;
pub mod hostel;
pub mod listcstring;
pub mod room;

use std::ffi::{CStr, CString, c_char};

fn string_to_const_char_ptr(str: String) -> *const c_char {
    let ret = CString::new(str).unwrap();
    Box::new(ret).into_raw() as *const c_char
}
pub unsafe fn const_char_ptr_to_string(ptr: *const c_char) -> String {
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        c_str
            .to_str()
            .expect("Could not convert from pointer to str")
            .to_string()
    }
}
