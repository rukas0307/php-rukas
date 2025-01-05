use ext_php_rs::prelude::*;
use std::net::TcpStream;

#[php_function(ignore_module)]
pub fn test_net() -> bool {
    TcpStream::connect("1.1.1.1:80").is_ok()
}
