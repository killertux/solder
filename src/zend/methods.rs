use super::internal_php_methods::*;
use super::types::*;
use std::ffi::CString;
use libc::*;
use std::ptr::null;

pub fn php_echo(message: &str) {
    let c_message = CString::new(message).unwrap();
    unsafe {
        php_printf(c_message.as_bytes_with_nul().as_ptr() as *const i8);
    }
}

pub trait PhpParseParameters {
    fn parse_parameters(self: &mut Self);
}

impl PhpParseParameters for [&mut Zval; 1] {
    fn parse_parameters(self: &mut Self) {
        let mut value_1 = ZendValue{long_value: 0};
        unsafe {
            zend_parse_parameters(
                1,
                c_str!("z"),
                &value_1,
            );
            add_zend_value_to_zval(value_1, self[0]);
        }
    }
}

impl PhpParseParameters for [&mut Zval; 2] {
    fn parse_parameters(self: &mut Self) {
        let mut value_1 = ZendValue{long_value: 0};
        let mut value_2 = ZendValue{long_value: 0};
        unsafe {
            zend_parse_parameters(
                2,
                c_str!("zz"),
                &value_1,
                &value_2,
            );
            add_zend_value_to_zval(value_1, self[0]);
            add_zend_value_to_zval(value_2, self[1]);
        }
    }
}

impl PhpParseParameters for [&mut Zval; 3] {
    fn parse_parameters(self: &mut Self) {
        let mut value_1 = ZendValue{long_value: 0};
        let mut value_2 = ZendValue{long_value: 0};
        let mut value_3 = ZendValue{long_value: 0};
        unsafe {
            zend_parse_parameters(
                3,
                c_str!("zzz"),
                &value_1,
                &value_2,
                &value_3,
            );
            add_zend_value_to_zval(value_1, self[0]);
            add_zend_value_to_zval(value_2, self[1]);
            add_zend_value_to_zval(value_3, self[2]);
        }
    }
}

impl PhpParseParameters for [&mut Zval; 4] {
    fn parse_parameters(self: &mut Self) {
        let mut value_1 = ZendValue{long_value: 0};
        let mut value_2 = ZendValue{long_value: 0};
        let mut value_3 = ZendValue{long_value: 0};
        let mut value_4 = ZendValue{long_value: 0};
        unsafe {
            zend_parse_parameters(
                4,
                c_str!("zzzz"),
                &value_1,
                &value_2,
                &value_3,
                &value_4,
            );
            add_zend_value_to_zval(value_1, self[0]);
            add_zend_value_to_zval(value_2, self[1]);
            add_zend_value_to_zval(value_3, self[2]);
            add_zend_value_to_zval(value_4, self[3]);
        }
    }
}

impl PhpParseParameters for [&mut Zval; 5] {
    fn parse_parameters(self: &mut Self) {
        let mut value_1 = ZendValue{long_value: 0};
        let mut value_2 = ZendValue{long_value: 0};
        let mut value_3 = ZendValue{long_value: 0};
        let mut value_4 = ZendValue{long_value: 0};
        let mut value_5 = ZendValue{long_value: 0};
        unsafe {
            zend_parse_parameters(
                5,
                c_str!("zzzzz"),
                &value_1,
                &value_2,
                &value_3,
                &value_4,
                &value_5,
            );
            add_zend_value_to_zval(value_1, self[0]);
            add_zend_value_to_zval(value_2, self[1]);
            add_zend_value_to_zval(value_3, self[2]);
            add_zend_value_to_zval(value_4, self[3]);
            add_zend_value_to_zval(value_5, self[4]);
        }
    }
}

fn add_zend_value_to_zval(value: ZendValue, zval: &mut Zval) {
    unsafe {
        let zval_from_value = *value.zval;
        zval.value = zval_from_value.value;
        zval.u1 = zval_from_value.u1;
        zval.u2 = zval_from_value.u2;
    }
}