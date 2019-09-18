use libc::*;
use std::ffi::{CString, CStr};
use std::slice;
use super::internal_php_methods::*;
use std::os::raw::c_void;
use std::ptr::null;

pub struct ExecuteData {}
pub struct ModuleDep {}

// Zend Types and Zval
//https://github.com/php/php-src/blob/d0754b86b1cb4774c4af64498641ddaaab745418/Zend/zend_types.h#L176-L233

pub enum InternalPhpTypes {
	UNDEF = 0,
	NULL = 1,
	FALSE = 2,
	TRUE = 3,
	LONG = 4,
	DOUBLE = 5,
	STRING = 6,
	ARRAY = 7,
	OBJECT = 8,
	RESOURCE = 9,
	REFERENCE = 10,
}

#[repr(C)]
pub union ZendValue {
	pub long_value: i64,
	pub double_value: f64,
	pub string: *mut ZendString,
	pub array: *mut ZendArray,
	pub zval: *mut Zval,
	pub void: *mut c_void,
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
pub struct Zval {
	pub value: ZendValue,
	pub u1: U1,
	pub u2: U2,
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
	pub len: usize,
	pub value: [u8; 0],
}

#[repr(C)]
pub struct Bucket {
	value: Zval,
	hash: u64,
	key: *mut ZendString,
}

#[repr(C)]
struct DtorFunc {}

#[repr(C)]
pub struct ZendArray {
	gc: ZendRefCounted,
	n_table_mask: u32,
	array_data: *mut Bucket,
	n_num_used: u32,
	n_num_of_elements: u32,
	n_table_size: u32,
	n_internal_pointer: u32,
	n_next_free_element: u64,
	p_destructor: DtorFunc,
}


impl ZendString {
	pub fn new_as_pointer(rust_str: &str) -> *mut ZendString {
		let c_format = CString::new(rust_str).unwrap();
		create_zend_string(rust_str.len(), c_format.as_bytes_with_nul().as_ptr() as *const i8)
	}
}

impl ZendArray {
	pub fn new_in_zval(zval: &mut Zval) {
		create_zend_array(zval);
	}

	pub fn add_value(array: &mut Zval, key: &mut Zval, value: &mut Zval) {
		unsafe {
			array_set_zval_key((*array).value.array, key, value);
		}
	}
}

impl Zval {
	pub fn new<T>(t: T) -> Self
		where Zval: From<T>
	{
		Zval::from(t)
	}

	pub fn new_as_null() -> Self {
		Zval {
			value: ZendValue {void: null::<c_void>() as *mut libc::c_void},
			u1: U1{type_info: InternalPhpTypes::NULL as u32},
			u2: U2{next: 0},
		}
	}

	pub fn is_undef(self: &Self) -> bool {
		unsafe {self.u1.type_info == InternalPhpTypes::UNDEF as u32}
	}

	pub fn is_null(self: &Self) -> bool {
		unsafe {self.u1.type_info == InternalPhpTypes::NULL as u32}
	}

	pub fn as_bool(self: &Self) -> Option<bool> {
		if unsafe {self.u1.type_info == InternalPhpTypes::TRUE as u32} {
			return Some(true);
		}
		if unsafe {self.u1.type_info == InternalPhpTypes::FALSE as u32} {
			return Some(false);
		}
		None
	}

	pub fn as_long(self: &Self) -> Option<i64> {
		if unsafe {self.u1.type_info == InternalPhpTypes::LONG as u32} {
			return Some(unsafe {self.value.long_value});
		}
		None
	}

	pub fn as_double(self: &Self) -> Option<f64> {
		if unsafe {self.u1.type_info == InternalPhpTypes::DOUBLE as u32} {
			return Some(unsafe {self.value.double_value});
		}
		None
	}

	pub fn as_string(self: &Self) -> Option<String> {
		if unsafe {self.u1.type_info == InternalPhpTypes::STRING as u32} {
			unsafe {
				let c_str = CStr::from_bytes_with_nul_unchecked(
					slice::from_raw_parts((*self.value.string).value.as_ptr(), (*self.value.string).len as usize + 1)
				);
				return match c_str.to_str() {
					Ok(str) => Some(str.to_string()),
					Err(_) => None,
				};
			}
		}
		None
	}
}

impl From<&str> for Zval {
	fn from(rust_str: &str) -> Self {
		Zval {
			value: ZendValue{string: ZendString::new_as_pointer(rust_str)},
			u1: U1{type_info: InternalPhpTypes::STRING as u32},
			u2: U2{next: 0}
		}
	}
}

impl From<String> for Zval {
	fn from(rust_string: String) -> Self {
		Zval::from(rust_string.as_str())
	}
}

impl From<i64> for Zval {
	fn from(number: i64) -> Self {
		Zval {
			value: ZendValue{long_value: number},
			u1: U1{type_info: InternalPhpTypes::LONG as u32},
			u2: U2{next: 0}
		}
	}
}

impl From<i32> for Zval {
	fn from(number: i32) -> Self {
		Zval {
			value: ZendValue{long_value: number as i64},
			u1: U1{type_info: InternalPhpTypes::LONG as u32},
			u2: U2{next: 0}
		}
	}
}

impl From<u32> for Zval {
	fn from(number: u32) -> Self {
		Zval {
			value: ZendValue{long_value: number as i64},
			u1: U1{type_info: InternalPhpTypes::LONG as u32},
			u2: U2{next: 0}
		}
	}
}

impl From<usize> for Zval {
	fn from(size: usize) -> Self {
		Zval {
			value: ZendValue{long_value: size as i64},
			u1: U1{type_info: InternalPhpTypes::LONG as u32},
			u2: U2{next: 0}
		}
	}
}

impl<T: Clone> From<Vec<T>> for Zval
	where Zval: From<T>
{
	fn from(vector: Vec<T>) -> Self {
		let mut returner = Zval::new_as_null();
		ZendArray::new_in_zval(&mut returner);
		for (index, value) in vector.into_iter().enumerate() {
			ZendArray::add_value(
				&mut returner,
				&mut Zval::new::<usize>(index),
				&mut Zval::new(value.clone())
			);
		}
		returner
	}
}
