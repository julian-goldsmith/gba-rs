pub const DISP_CNT: *mut u32 = 0x04000000 as _;
pub const REG_VCOUNT: *mut u16 = 0x04000006 as _;
pub const BG0_CNT: *mut u32 = 0x04000008 as _;
pub const MEM_VRAM: *mut u32 = 0x06000000 as _;

pub const TILE_MEM: *mut u32 = 0x06000000 as _;
pub const BG0_MEM: *mut u32 = 0x0600F000 as _;
pub const BG_PAL_MEM: *mut u32 = 0x05000000 as _;

pub const OBJ_CHAR_MEM: *mut u32 = 0x06010000 as _;
pub const OBJ_PAL_MEM: *mut u32 = 0x05000200 as _;

pub const BG_8BPP: u32 = 0b10000000;
pub const BG_REG_64X32: u32 = 0b100000000000000;

fn set_bit_if(condition: bool, n: u32) -> u32 {
    if condition {
        1 << n
    } else {
        0
    }
}

pub fn build_bgcnt(size: u32, overflow: bool, sbb: u32, is_256_colors: bool, 
                      is_mosaic: bool, cbb: u32, prio: u32) -> u32 {
    size << 14 | set_bit_if(overflow, 13) | sbb << 8 |
        set_bit_if(is_256_colors, 7) | set_bit_if(is_mosaic, 6) |
        cbb << 2 | prio
}

pub const DCNT_MODE0: u32 = 0b00;
pub const DCNT_BG0: u32 = 0b100000000;
pub const DCNT_OBJ: u32 = 0x1000;

pub const OAM_MEM: *mut u32 = 0x07000000 as _;
