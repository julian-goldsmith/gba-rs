#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

pub mod runtime;

#[no_mangle]
pub extern "C" fn main() {
    loop { }
}
