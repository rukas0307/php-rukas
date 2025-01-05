#![allow(unused_imports, dead_code)]

// External PHP-RS library imports
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;

// Internal module declarations
mod zval;

// Features
#[cfg(not(feature = "disable_network"))]
mod network;
#[cfg(not(feature = "disable_network"))]
use crate::network::{
    test_net::_internal_php_test_net, 
    trace::_internal_php_trace,
    _0x0::_internal_php__0x0,
};

// PHP Rukas extension entry point
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
