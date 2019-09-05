use super::types::*;
use libc::*;
use std::os::raw::c_void;

extern "C" {
    pub fn strpprintf(max_len: size_t, format: *const c_char) -> *mut ZendString;
    pub fn zend_parse_parameters(num_args: i32, format: *const c_char, ...) -> i32;
    pub fn _array_init(arg: *mut Zval, size: u32) -> i32;
    pub fn array_set_zval_key(ht: *mut ZendArray, key: *mut Zval, value: *mut Zval) -> i32;
    pub fn php_printf(format: *const c_char , ...) -> size_t;
}