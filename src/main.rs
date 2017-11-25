#![feature(lang_items)]
#![no_std]
#![crate_type="staticlib"]

pub mod rlibc;
pub mod runtime;
mod gba;

use core::ptr;

extern "C" {
    static tilesTiles: *const u32;
    static tilesMap: *const u32;
    static tilesPal: *const u32;
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    ptr::copy(tilesPal, gba::BG_PAL_MEM, 128);
    ptr::copy(tilesTiles, gba::TILE_MEM, 4680);
    ptr::copy(tilesMap, gba::BG0_MEM, 300);

    *gba::DISP_CNT = gba::DCNT_MODE0 | gba::DCNT_BG0;

    *gba::BG0_CNT = 0x1E80;//gba::BG_8BPP | gba::BG_REG_64X32;

    loop { }
}
