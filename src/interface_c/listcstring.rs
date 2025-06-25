use std::ffi::{CStr, c_char, c_uint};

#[repr(C)]
pub struct ListCString {
    pub strings: *const *const c_char,
    pub length: c_uint,
}
impl ListCString {
    pub fn to_vec(&self) -> Vec<String> {
        if self.strings.is_null() || self.length == 0 {
            return Vec::new();
        }
        let cities_ptrs = unsafe { std::slice::from_raw_parts(self.strings, self.length as usize) };

        let mut result = Vec::with_capacity(cities_ptrs.len());

        for ptr in cities_ptrs.iter() {
            let string = unsafe { CStr::from_ptr(*ptr) }.to_str().unwrap();
            result.push(string.to_string());
        }

        result
    }

    // fn from_vec(vec: Vec<String>) -> ListCString {
    //     if vec.is_empty() {
    //         return ListCString {
    //             strings: std::ptr::null(),
    //             length: 0,
    //         };
    //     }

    //     let c_strings = vec
    //         .iter()
    //         .map(|string| CString::new(string.as_str()).unwrap())
    //         .collect::<Vec<CString>>();

    //     let len = c_strings.len();
    //     let mut c_strings = c_strings;
    //     let ptr = c_strings.as_mut_ptr();
    //     std::mem::forget(c_strings);

    //     ListCString {
    //         strings: ptr,
    //         length: len as u32,
    //     }
    // }
}
