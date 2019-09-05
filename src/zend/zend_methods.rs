use super::types::*;
use libc::*;
use std::os::raw::c_void;

extern "C" {
    pub fn strpprintf(max_len: size_t, format: *const c_char) -> *mut ZendString;
    pub fn zend_parse_parameters(num_args: i32, format: *const c_char, ...) -> i32;
}