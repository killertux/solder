
#[macro_export]
macro_rules! c_str {
    ($s:expr) => { {
        concat!($s, "\0").as_ptr() as *const c_char
    } }
}

#[macro_export]
macro_rules! php_return {
    ($retval:expr, $value:expr) => {
        (*$retval) = Zval::new($value);
    };
}
