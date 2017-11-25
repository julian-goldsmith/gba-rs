#![feature(lang_items)]
#![no_std]
#![crate_type="staticlib"]

pub mod runtime;
mod gba;

use core::ptr;

extern "C" {
    static tilesTiles: [u32; 1];        // NOTE: not the real length
    static tilesMap: [u32; 1];      // NOTE: not the real length
    static tilesPal: [u32; 1];      // NOTE: not the real length
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    ptr::copy(&tilesTiles as *const u32, gba::TILE_MEM, 9376);
    ptr::copy(&tilesPal as *const u32, gba::BG_PAL_MEM, 128);
    ptr::copy(&tilesMap as *const u32, gba::BG0_MEM, 512);

    *gba::BG0_CNT = 0x1E80;//gba::BG_8BPP | gba::BG_REG_64X32;
    *gba::DISP_CNT = gba::DCNT_MODE0 | gba::DCNT_BG0;

    loop { }
}
