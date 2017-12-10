#![feature(lang_items)]
#![no_std]
#![crate_type="staticlib"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod runtime;
mod gba;
mod sprite;
mod ctypes;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use core::ptr;

unsafe fn wait_vsync() {
    while ptr::read_volatile(gba::REG_VCOUNT) < 160 { }
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    ptr::copy(&tilesTiles, gba::TILE_MEM as _, 1);
    ptr::copy(&tilesTiles, gba::OBJ_CHAR_MEM as _, 1);
    ptr::copy(&tilesPal, gba::BG_PAL_MEM as _, 1);
    ptr::copy(&tilesPal, gba::OBJ_PAL_MEM as _, 1);
    ptr::copy(&tilesMap, gba::BG0_MEM as _, 1);

    //*gba::BG0_CNT = 0x1E80;//gba::BG_8BPP | gba::BG_REG_64X32;
    *gba::BG0_CNT = gba::build_bgcnt(0, false, 0, true, false, 30, 0);
    *gba::DISP_CNT = gba::DCNT_MODE0 | gba::DCNT_BG0 | gba::DCNT_OBJ;

    let entry = sprite::OAMEntry::new(16, 16, 200);

    loop {
        wait_vsync();
        entry.oam_copy();
    };
}
