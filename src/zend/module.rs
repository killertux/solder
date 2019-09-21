use std;
use std::mem;
use libc::*;

use super::types::*;

type StartupFunc = extern fn (type_: c_int, module_number: c_int) -> c_int;
type ShutdownFunc = extern fn (type_: c_int, module_number: c_int) -> c_int;
type InfoFunc = extern fn () ;
type GlobalsCtorFunc = extern fn (global: *const c_void) -> c_void;
type GlobalsDtorFunc = extern fn (global: *const c_void) -> c_void;
type PostDeactivateFunc = extern fn () -> c_int;
type HandlerFunc = extern fn (execute_data: &ExecuteData, retval: &mut Zval);

#[repr(C)]
pub struct ArgInfo {
	name: *const c_char,
	class_name: *const c_char,
	type_hint: c_char,
	pass_by_reference: c_char,
	allow_null: c_char,
	is_variadic: c_char,
}

/// Information about the arguments of a function
impl ArgInfo {
	pub fn new(name: *const c_char, allow_null: c_char, is_variadic: c_char, by_reference: c_char) -> ArgInfo {
		ArgInfo {
			name: name,
			class_name: std::ptr::null(),
			type_hint: 0,
			pass_by_reference: by_reference,
			allow_null: allow_null,
			is_variadic: is_variadic,
		}
	}
}

/// Struct with the functions that will be available inside PHP
#[repr(C)]
pub struct Function {
	fname: *const c_char,
	handler: Option<HandlerFunc>,
	arg_info: *const ArgInfo,
	num_args: u32,
	flags: u32,
}

impl Function {
	pub fn end() -> Function {
		Function {
			fname: std::ptr::null(),
			handler: None,
			arg_info: std::ptr::null(),
			num_args: 0,
			flags: 0,
		}
	}
}

pub struct FunctionBuilder {
	function: Function,
	args: Vec<ArgInfo>,
}

impl FunctionBuilder {
	/// Create a function with name
	pub fn new(name: *const c_char, handler: HandlerFunc) -> Self {
		FunctionBuilder {
			function: Function {
				fname: name,
				handler: Some(handler),
				arg_info: std::ptr::null(),
				num_args: 0,
				flags: 0,
			},
			args: Vec::new(),
		}
	}

	/// Add an argument to the function
	pub fn with_arg(mut self, arg: ArgInfo) -> Self {
		self.args.push(arg);
		self
	}

	/// Build the function
	pub fn build(mut self)-> Function {
		if self.args.is_empty() {
			return self.function;
		}
		self.function.num_args = self.args.len() as u32 - 1;
		self.function.arg_info = Box::into_raw(self.args.into_boxed_slice()) as *mut ArgInfo;
		self.function
	}
}

pub struct INI {}

/// Module represents your extension
#[repr(C)]
pub struct Module {
	size: c_ushort,
	zend_api: c_uint,
	zend_debug: c_uchar,
	zts: c_uchar,
	ini_entry: *const INI,
	deps: *const ModuleDep,
	name: *const c_char,
	functions: *const Function,
	module_startup_func: Option<StartupFunc>,
	module_shutdown_func: Option<ShutdownFunc>,
	request_startup_func: Option<StartupFunc>,
	request_shutdown_func: Option<ShutdownFunc>,
	info_func: Option<InfoFunc>,
	version: *const c_char,
	globals_size: size_t,
	globals_ptr: *const c_void,
	globals_ctor: Option<GlobalsCtorFunc>,
	globals_dtor: Option<GlobalsDtorFunc>,
	post_deactivate_func: Option<PostDeactivateFunc>,
	module_started: c_int,
	type_: c_uchar,
	handle: *const c_void,
	module_number: c_int,
	build_id: *const c_char,
}

impl Module {
	pub fn into_raw(self) -> *mut Self {
		Box::into_raw(Box::new(self))
	}
}

pub struct ModuleBuilder {
	module: Module,
	functions: Vec<Function>,
}

impl ModuleBuilder {
	/// Create a module with name and version
	pub fn new(name: *const c_char, version: *const c_char) -> ModuleBuilder {
		ModuleBuilder {
			module: Module {
				size: mem::size_of::<Module>() as u16,
				zend_api: env!("PHP_API_VERSION").parse::<u32>().unwrap(),
				zend_debug: 0,
				zts: 0,
				ini_entry: std::ptr::null(),
				deps: std::ptr::null(),
				name: name,
				functions: std::ptr::null(),
				module_startup_func: None,
				module_shutdown_func: None,
				request_startup_func: None,
				request_shutdown_func: None,
				info_func: None,
				version: version,
				globals_size: 0,
				globals_ptr: std::ptr::null(),
				globals_ctor: None,
				globals_dtor: None,
				post_deactivate_func: None,
				module_started: 0,
				type_: 0,
				handle: std::ptr::null(),
				module_number: 0,
				build_id: c_str!(env!("PHP_EXTENSION_BUILD")),
			},
			functions: Vec::new(),
		}
	}

	/// Set a startup function
	pub fn with_startup_function(mut self, func: StartupFunc) -> Self {
		self.module.module_startup_func = Some(func);
		self
	}

	/// Set a shutdown function
	pub fn with_shutdown_function(mut self, func: ShutdownFunc) -> Self {
		self.module.module_shutdown_func = Some(func);
		self
	}

	/// Set a function to print information in PHP Info
	pub fn with_info_function(mut self, func: InfoFunc) -> Self {
		self.module.info_func = Some(func);
		self
	}

	/// Set functions that will be available from PHP.
	pub fn with_function(mut self, function: Function) -> Self {
		self.functions.push(function);
		self
	}

	pub fn build(mut self) -> Module {
		self.functions.push(Function::end());
		self.module.functions = Box::into_raw(self.functions.into_boxed_slice()) as *const Function;
		self.module
	}
}

unsafe impl Sync for Module {}
