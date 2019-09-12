use std::env;
use std::process::Command;

const PHP_API_VERSION_COMMAND: &str = "php -i | grep 'PHP API' | sed -e 's/PHP API => //'";
const PHP_API_VERSION_ERROR: &str = "Error trying to get PHP API version from your machine";
const PHP_EXTENSION_BUILD_COMMAND: &str = "php -i | grep 'PHP Extension Build' | sed -e 's/PHP Extension Build => //'";
const PHP_EXTENSION_BUILD_ERROR: &str = "Error trying to get PHP EXTENSION BUILD version from your machine";

fn main() {
    let api_version = match env::var("PHP_API_VERSION") {
        Ok(api_version) => api_version,
        Err(_) => execute_command(PHP_API_VERSION_COMMAND, PHP_API_VERSION_ERROR)
    };
    let zend_extension_build = match env::var("PHP_EXTENSION_BUILD") {
        Ok(zend_extension_build) => zend_extension_build,
        Err(_) => execute_command(PHP_EXTENSION_BUILD_COMMAND, PHP_EXTENSION_BUILD_ERROR)
    };

    println!("cargo:rustc-env=PHP_API_VERSION={}", api_version);
    println!("cargo:rerun-if-env-changed=PHP_API_VERSION");
    println!("cargo:rustc-env=PHP_EXTENSION_BUILD={}", zend_extension_build);
    println!("cargo:rerun-if-env-changed=PHP_EXTENSION_BUILD");
    set_version_features(api_version);
}

fn execute_command(command: &str, error_message: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect(error_message)
        .stdout;
    String::from_utf8(output).unwrap()
}

fn set_version_features(api_version: String) {
    if api_version.trim().parse::<i64>().unwrap() >= 20170718 {
        println!("cargo:rustc-cfg=feature=\"php72\"");
    }
}
