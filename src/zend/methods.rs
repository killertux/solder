use super::internal_php_methods::*;
use super::types::*;
use std::ffi::CString;
use libc::*;

pub fn php_echo(message: &str) {
    let c_message = CString::new(message).unwrap();
    unsafe {
        php_printf(c_message.as_bytes_with_nul().as_ptr() as *const i8);
    }
}

pub fn parse_parameters(parameter_1: &mut Zval) {
    unsafe {
        zend_parse_parameters(
            1,
            c_str!("z"),
            &parameter_1
        );
    }
}