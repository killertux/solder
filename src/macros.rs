
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

#[macro_export]
macro_rules! php_parse_parameters {
	($p1:expr) => {
		[$p1].parse_parameters();
	};
	($p1:expr, $($rest:expr), *) => {
		[$p1, $($rest), *].parse_parameters();
	}
}
