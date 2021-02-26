use libc::*;
use std::ffi::CString;

/// Module with functions to print a PHPInfo table
///
/// ```
/// use php::info::*;
/// print_table_start();
/// print_table_row("gpg_keys", "enabled");
/// print_table_end();
/// ```

extern {
    pub fn php_info_print_table_start();
    pub fn php_info_print_table_row(num_cols: c_int, ...) -> c_void;
    pub fn php_info_print_table_header(num_cols: c_int, ...) -> c_void;
    pub fn php_info_print_table_end();
}

/// Starts the PHP Info entry
pub fn print_table_start() {
    unsafe { php_info_print_table_start() };
}

/// Print a row with info.
pub fn print_table_row(key: &str, value: &str) {
    let v1 = CString::new(key).unwrap();
    let v2 = CString::new(value).unwrap();
    unsafe {
        php_info_print_table_row(2, v1.as_ptr(), v2.as_ptr());
    };
}

/// Print the header of the PHP info table.
pub fn print_table_header(key: &str, value: &str) {
    let v1 = CString::new(key).unwrap();
    let v2 = CString::new(value).unwrap();
    unsafe {
        php_info_print_table_header(2, v1.as_ptr(), v2.as_ptr());
    };
}

/// Ends the table
pub fn print_table_end() {
    unsafe { php_info_print_table_end() }
}

