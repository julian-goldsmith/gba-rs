#![feature(lang_items)]
#![no_std]
#![crate_type="staticlib"]

pub mod runtime;
mod gba;
mod sprite;

use core::ptr;

extern "C" {
    static tilesTiles: [u32; 1];    // NOTE: not the real length
    static tilesMap: [u32; 1];      // NOTE: not the real length
    static tilesPal: [u32; 1];      // NOTE: not the real length
}

unsafe fn wait_vsync() {
    while ptr::read_volatile(gba::REG_VCOUNT) < 160 { }
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    ptr::copy(&tilesTiles as *const u32, gba::TILE_MEM, 9376);
    ptr::copy(&tilesTiles as *const u32, gba::OBJ_CHAR_MEM, 9376);
    ptr::copy(&tilesPal as *const u32, gba::BG_PAL_MEM, 128);
    ptr::copy(&tilesPal as *const u32, gba::OBJ_PAL_MEM, 128);
    ptr::copy(&tilesMap as *const u32, gba::BG0_MEM, 512);

    *gba::BG0_CNT = 0x1E80;//gba::BG_8BPP | gba::BG_REG_64X32;
    *gba::DISP_CNT = gba::DCNT_MODE0 | gba::DCNT_BG0 | gba::DCNT_OBJ;

    let entry = sprite::OAMEntry::new(16, 16, 200);

    loop {
        wait_vsync();
        entry.oam_copy();
    };
}
