use libc::*;
use std::ffi::CString;
use super::zend_methods::*;
use std::os::raw::c_void;

pub struct ExecuteData {}
pub struct ModuleDep {}

// Zend Types and Zval
//https://github.com/php/php-src/blob/d0754b86b1cb4774c4af64498641ddaaab745418/Zend/zend_types.h#L176-L233
#[repr(C)]
pub union ZendValue {
	pub long_value: c_long,
	pub double_value: c_double,
	pub string: *mut ZendString,
}

#[repr(C)]
pub union U1 {
	pub type_info: u32,
}

#[repr(C)]
pub union U2 {
	pub next: u32,
}

#[repr(C)]
pub struct ZendRefCounted {
	pub ref_count: u32,
	pub type_info: u32,
}

#[repr(C)]
pub struct ZendString {
	pub gc: ZendRefCounted,
	pub hash: u32,
	pub len: libc::size_t,
	pub value: *mut libc::c_char,
}

#[repr(C)]
pub struct Zval {
	pub value: ZendValue,
	pub u1: U1,
	pub u2: U2,
}

fn zend_string(max_len: libc::size_t, format: &str) -> *mut ZendString {
	let c_format = CString::new(format).unwrap();
	unsafe {
		strpprintf(max_len, c_format.as_bytes_with_nul().as_ptr() as *const i8)
	}
}

impl From<&str> for ZendValue {
	fn from(rust_str: &str) -> Self {
		ZendValue {
			string: zend_string(rust_str.len(), rust_str),
		}
	}
}

pub trait IntoZval {
	fn into_zval(self, zval: &mut Zval);
}

impl IntoZval for &str {
	fn into_zval(self, zval: &mut Zval) {
		(*zval).u1.type_info = 6;
		(*zval).value = ZendValue::from(self);
	}
}

impl IntoZval for i64 {
	fn into_zval(self, zval: &mut Zval) {
		(*zval).u1.type_info = 4;
		(*zval).value.long_value = self;
	}
}
