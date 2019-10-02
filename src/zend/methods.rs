use super::internal_php_methods::*;
use super::types::*;
use std::ffi::CString;

/// As the name suggests, this method is acts like a PHP echo
/// ```
/// php_echo("hello world");
/// ```
pub fn php_echo(message: &str) {
    let c_message = CString::new(message).unwrap();
    unsafe {
        php_printf(c_message.as_bytes_with_nul().as_ptr() as *const i8);
    }
}

/// Executes a callable. The first parameter is the Zval with the callable. The second parameter
/// is an array containing the callable parameters. It returns the callable return.
///
/// This method does not checks if the callable is an actually callable. If you pass a Zval that is
/// not a callable PHP may throw an warning.
/// ```
/// use solder::zend::execute_callable;
/// let mut callable = Zval::new_as_null();
/// php_parse_parameters!(&mut callable);
/// execute_callable(callable, &mut [Zval::from("Hello World")]);
/// ```
pub fn execute_callable(callable: &mut Zval, params: &mut [Zval]) -> Zval {
    let mut returner = Zval::new_as_null();
    unsafe{
        _call_user_function_ex(callable, &mut Zval::from(zend_get_callable_name(callable)), &mut returner, params.len() as u32, params.as_mut_ptr(), 0)
    };
    returner
}

/// This macro parses all parameters passed to function. Currently, there is a limit of 5 parameters.
/// If you try to get more parameters than what were passed to the function, PHP will emit a Warning
/// and the excess zvals will be undefined.
///
/// ```
/// use solder::zend::{ExecuteData, Zval, FromPhpZval};
/// #[no_mangle]
/// pub extern fn hello_world(_data: &ExecuteData, retval: &mut Zval) {
///     let mut name_zval = Zval::new_as_null();
///     php_parse_parameters!(&mut name_zval);
///     let name = String::try_from(name_zval).ok().unwrap();
///     let hello = format!("Hello {}", name);
///     php_return!(retval, hello);
/// }
/// ```
#[macro_export]
macro_rules! php_parse_parameters {
	($p1:expr) => {
		[$p1].parse_parameters();
	};
	($p1:expr, $($rest:expr), *) => {
		[$p1, $($rest), *].parse_parameters();
	}
}
pub trait PhpParseParameters {
    fn parse_parameters(self: &mut Self);
}

impl PhpParseParameters for [&mut Zval; 1] {
    fn parse_parameters(self: &mut Self) {
        let value_1 = ZendValue{long_value: 0};
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
        let value_1 = ZendValue{long_value: 0};
        let value_2 = ZendValue{long_value: 0};
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
        let value_1 = ZendValue{long_value: 0};
        let value_2 = ZendValue{long_value: 0};
        let value_3 = ZendValue{long_value: 0};
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
        let value_1 = ZendValue{long_value: 0};
        let value_2 = ZendValue{long_value: 0};
        let value_3 = ZendValue{long_value: 0};
        let value_4 = ZendValue{long_value: 0};
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
        let value_1 = ZendValue{long_value: 0};
        let value_2 = ZendValue{long_value: 0};
        let value_3 = ZendValue{long_value: 0};
        let value_4 = ZendValue{long_value: 0};
        let value_5 = ZendValue{long_value: 0};
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
        zval.type_info = zval_from_value.type_info;
        zval.u2 = zval_from_value.u2;
    }
}