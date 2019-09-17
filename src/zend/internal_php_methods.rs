use super::types::*;
use libc::*;
use std::os::raw::c_void;

extern "C" {
    pub fn zend_parse_parameters(num_args: i32, format: *const c_char, ...) -> i32;
    pub fn zend_parse_parameter(flag: i32, arg_num: i32, format: *const c_char, ...) -> i32;
    pub fn _array_init(arg: *mut Zval, size: u32) -> i32;
    pub fn array_set_zval_key(ht: *mut ZendArray, key: *mut Zval, value: *mut Zval) -> i32;
    pub fn php_printf(format: *const c_char , ...) -> size_t;
}

#[cfg(feature = "php72")]
extern "C" {
    fn zend_strpprintf(max_len: size_t, format: * const c_char) -> * mut ZendString;
}

#[cfg(feature = "php72")]
pub fn create_zend_string(size: size_t, string: *const c_char) -> * mut ZendString {
    unsafe {
        zend_strpprintf(size, string)
    }
}

#[cfg(not(feature = "php72"))]
extern "C" {
    fn strpprintf(max_len: size_t, format: *const c_char) -> *mut ZendString;
}

#[cfg(not(feature = "php72"))]
pub fn create_zend_string(size: size_t, string: *const c_char) -> * mut ZendString {
    unsafe {
        strpprintf(size, string)
    }
}
