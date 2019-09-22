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
pub union TypeInfoUnion {
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
struct DtorFunc {void: *mut c_void}

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

/// Zval is the basic struct that PHP uses to store variables.
/// Since PHP is not a strict type language, the type of a zval holds what type is currently stored
/// and this value can change.
#[repr(C)]
pub struct Zval {
	pub value: ZendValue,
	pub type_info: TypeInfoUnion,
	pub u2: U2,
}

impl Zval {
	/// Creates a new zval and store the value in it. It is actually just a wrapper for Zval::From<T>
	pub fn new<T>(value: T) -> Self
		where Zval: From<T>
	{
		Zval::from(value)
	}

	/// Create a zval as null.
	/// This null is the PHP type null and not a `None`
	pub fn new_as_null() -> Self {
		Zval {
			value: ZendValue {void: null::<c_void>() as *mut libc::c_void},
			type_info: TypeInfoUnion {type_info: InternalPhpTypes::NULL as u32},
			u2: U2{next: 0},
		}
	}

	/// Returns if a zval is undefined.
	/// Undefined means that this zval holds no value
	pub fn is_undef(self: &Self) -> bool {
		self.type_info.is_from_type(InternalPhpTypes::UNDEF)
	}

	/// Returns if a zval is null
	pub fn is_null(self: &Self) -> bool {
		self.type_info.is_from_type(InternalPhpTypes::NULL)
	}

	/// Returns if a zval is a integer (i64)
	pub fn is_integer(self: &Self) -> bool {
		self.type_info.is_from_type(InternalPhpTypes::NULL)
	}

	/// Returns if a zval is a float (f64)
	pub fn is_float(self: &Self) -> bool {
		self.type_info.is_from_type(InternalPhpTypes::NULL)
	}

	/// Returns if a zval is string
	pub fn is_string(self: &Self) -> bool {
		self.type_info.is_from_type(InternalPhpTypes::NULL)
	}

	/// Returns if a zval is array (Vec<>)
	pub fn is_array(self: &Self) -> bool {
		self.type_info.is_from_type(InternalPhpTypes::NULL)
	}
}

/// Returns a value from you function back to PHP.
/// You need to pass the retval from the function parameter and the value that you want to return.
///
/// ```
/// use solder::zend::{ExecuteData, Zval};
/// #[no_mangle]
/// pub extern fn hello_world(_data: &ExecuteData, retval: &mut Zval) {
///    php_return!(retval, "Hello World!");
///}
/// ```
#[macro_export]
macro_rules! php_return {
    ($retval:expr, $value:expr) => {
        (*$retval) = Zval::new($value);
    };
}

impl From<&str> for Zval {
	fn from(rust_str: &str) -> Self {
		Zval {
			value: ZendValue{string: ZendString::new_as_pointer(rust_str)},
			type_info: TypeInfoUnion {type_info: InternalPhpTypes::STRING as u32},
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
			type_info: TypeInfoUnion {type_info: InternalPhpTypes::LONG as u32},
			u2: U2{next: 0}
		}
	}
}

impl From<i32> for Zval {
	fn from(number: i32) -> Self {
		Zval {
			value: ZendValue{long_value: number as i64},
			type_info: TypeInfoUnion {type_info: InternalPhpTypes::LONG as u32},
			u2: U2{next: 0}
		}
	}
}

impl From<u32> for Zval {
	fn from(number: u32) -> Self {
		Zval {
			value: ZendValue{long_value: number as i64},
			type_info: TypeInfoUnion {type_info: InternalPhpTypes::LONG as u32},
			u2: U2{next: 0}
		}
	}
}

impl From<usize> for Zval {
	fn from(size: usize) -> Self {
		Zval {
			value: ZendValue{long_value: size as i64},
			type_info: TypeInfoUnion {type_info: InternalPhpTypes::LONG as u32},
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

/// This clone is not safe because we are copying the pointers and not the values.
/// The PHP GC will (probably) not deallocate this parameters as long as we stay single threaded.
/// But, I do need to improve this implementation
impl Clone for Zval {
	fn clone(&self) -> Self {
		Zval {
			value: ZendValue{long_value: unsafe{self.value.long_value}},
			type_info: TypeInfoUnion {type_info: unsafe{self.type_info.type_info}},
			u2: U2{next: unsafe{self.u2.next}},
		}
	}
}

impl TypeInfoUnion {
	fn is_from_type(self: &Self, php_type: InternalPhpTypes) -> bool {
		unsafe {self.type_info & 0x000F == php_type as u32}
	}
}

/// Errors that are thrown if you try to convert a Zval to a different type than it's value
pub enum PhpTypeConversionError {
	NotBool(TypeInfoUnion),
	NotInteger(TypeInfoUnion),
	NotFloat(TypeInfoUnion),
	NotString(TypeInfoUnion),
	NotArray(TypeInfoUnion),
}

pub trait FromPhpZval: Sized {
	fn try_from(value: Zval) -> Result<Self, PhpTypeConversionError>;
}

impl FromPhpZval for bool {
	fn try_from(zval: Zval) -> Result<Self, PhpTypeConversionError> {
		if zval.type_info.is_from_type(InternalPhpTypes::TRUE) {
			return Ok(true);
		}
		if zval.type_info.is_from_type(InternalPhpTypes::FALSE) {
			return Ok(false);
		}
		Err(PhpTypeConversionError::NotBool(zval.type_info))
	}
}

impl FromPhpZval for i64 {
	fn try_from(zval: Zval) -> Result<Self,PhpTypeConversionError> {
		if zval.type_info.is_from_type(InternalPhpTypes::LONG) {
			return Ok(unsafe {zval.value.long_value});
		}
		Err(PhpTypeConversionError::NotInteger(zval.type_info))
	}
}

impl FromPhpZval for f64 {
	fn try_from(zval: Zval) -> Result<Self, PhpTypeConversionError> {
		if zval.type_info.is_from_type(InternalPhpTypes::DOUBLE) {
			return Ok(unsafe {zval.value.double_value});
		}
		Err(PhpTypeConversionError::NotFloat(zval.type_info))
	}
}

impl FromPhpZval for String {
	fn try_from(zval: Zval) -> Result<Self, PhpTypeConversionError> {
		if !zval.type_info.is_from_type(InternalPhpTypes::STRING) {
			return Err(PhpTypeConversionError::NotString(zval.type_info));
		}
		unsafe {
			let c_str = CStr::from_bytes_with_nul_unchecked(
				slice::from_raw_parts((*zval.value.string).value.as_ptr(), (*zval.value.string).len as usize + 1)
			);
			return match c_str.to_str() {
				Ok(str) => Ok(str.to_string()),
				//Not a very good error.
				Err(_) => Err(PhpTypeConversionError::NotString(zval.type_info)),
			};
		}
	}
}

impl <T: FromPhpZval> FromPhpZval for Vec<T> {
	fn try_from(zval: Zval) -> Result<Self, PhpTypeConversionError> {
		if !zval.type_info.is_from_type(InternalPhpTypes::ARRAY) {
			return Err(PhpTypeConversionError::NotArray(zval.type_info));
		}
		let mut returner: Vec<T> = Vec::new();
		let table_size = unsafe {(*zval.value.array).n_table_size};
		for index in 0..table_size {
			let cloned_value = unsafe {(*(*zval.value.array).array_data.offset(index as isize)).value.clone()};
			if !cloned_value.type_info.is_from_type(InternalPhpTypes::UNDEF) {
				returner.push(T::try_from(cloned_value)?);
			}
		}
		Ok(returner)
	}
}