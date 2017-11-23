#![feature(lang_items)]
#![no_std]
#![crate_type="staticlib"]

pub mod runtime;
mod gba;

#[no_mangle]
pub unsafe extern "C" fn main() {
    *gba::DISP_CNT = 0x0403;

    let arr = 0x06000000 as *mut [u16; 160 * 240];

    (*arr)[120 + 80*240] = 0x001F;
    (*arr)[136 + 80*240] = 0x03E0;
    (*arr)[120 + 96*240] = 0x7C00;

    loop { }
}
