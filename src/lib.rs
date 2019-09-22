//! ### Solder your Rust into PHP
//!
//! This library tries to help you improve your PHP application using extensions written in Rust.
//!
//! The idea is to be able to write code purely in rust, compile it using cargo and load the library direct into PHP. It started as a fork from `php-rs`.
//!
//!
//! Note that this is the first version of this crate and a lot of things will change. Also, I can't stress enough how unstable this crate is right now. Use it with caution.
//!
//! Example:
//!
//! ```rust
//! extern crate libc;
//! extern crate solder;
//!
//! use solder::*;
//! use solder::zend::*;
//! use solder::info::*;
//!
//! #[no_mangle]
//! pub extern fn php_module_info() {
//!     print_table_start();
//!     print_table_row("A demo PHP extension written in Rust", "enabled");
//!     print_table_end();
//! }
//!
//! #[no_mangle]
//! pub extern fn get_module() -> *mut zend::Module {
//!     let function = FunctionBuilder::new(c_str!("hello_world"), hello_world)
//!         .with_arg(ArgInfo::new(c_str!("name"), 0, 0, 0))
//!         .build();
//!     ModuleBuilder::new(c_str!("hello_world"), c_str!("0.1.0-dev"))
//!         .with_info_function(php_module_info)
//!         .with_function(function)
//!         .build()
//!         .into_raw()
//! }
//!
//!
//! #[no_mangle]
//! pub extern fn hello_world(_data: &ExecuteData, retval: &mut Zval) {
//!     let mut name_zval = Zval::new_as_null();
//!     php_parse_parameters!(&mut name_zval);
//!     let name = String::try_from(name_zval).ok().unwrap();
//!     let hello = format!("Hello {}", name);
//!     php_return!(retval, hello);
//! }
//! ```
//!
//! To compile it, we need to add to our `.cargo/config`:
//! ```
//! [build]
//! rustflags = ["-C", "link-arg=-Wl,-undefined,dynamic_lookup"]
//! ```
//!
//! Than, you compile the extension using `cargo build` and load it copying it to your PHP modules dir and modifying you `php.ini`.
//!
//! ```
//! $ cargo build && php -dextension=$(pwd)/target/debug/libhelloworld.so -a
//!    Compiling solder v0.1.0 (/src)
//!    Compiling helloworld v0.1.0 (/src/examples/helloworld)
//!     Finished dev [unoptimized + debuginfo] target(s) in 5.93s
//!  Interactive shell
//!
//! php > var_dump(hello_world("Bruno"));
//! string(11) "Hello Bruno"
//! php >
//! ```
//! ### PHP Versions
//! For now, this crate only works with PHP7.
//! During the build, it tries to get the PHP API VERSION and PHP EXTENSION BUILD from the installed PHP. But, you can compile for other versions by manually setting the envs PHP_API_VERSION and PHP_EXTENSION_BUILD
//!
//! If you have questions or ideas to the project. Feel free to contact me.

extern crate libc;

#[macro_use]
pub mod macros;
pub mod zend;
pub mod info;
