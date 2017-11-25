pub const DISP_CNT: *mut u32 = 0x04000000 as *mut u32;
pub const BG0_CNT: *mut u32 = 0x04000008 as *mut u32;
pub const MEM_VRAM: *mut u32 = 0x06000000 as *mut u32;

pub const TILE_MEM: *mut u32 = 0x06000000 as *mut u32;
pub const BG0_MEM: *mut u32 = 0x0600F000 as *mut u32;
pub const BG_PAL_MEM: *mut u32 = 0x05000000 as *mut u32;

pub const BG_8BPP: u32 = 0b10000000;
pub const BG_REG_64X32: u32 = 0b100000000000000;

pub const DCNT_MODE0: u32 = 0b00;
pub const DCNT_BG0: u32 = 0b100000000;
