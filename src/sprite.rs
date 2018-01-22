use gba;
use core::ptr;

#[repr(C)]
pub struct OAMEntry {
    pub attr0: u16,
    pub attr1: u16,
    pub attr2: u16,
    pub pad: u16,
}

impl OAMEntry {
    pub fn new(x: u16, y: u8, tile_id: u16) -> OAMEntry {
        let mut entry = OAMEntry { attr0: 1 << 13, attr1: 0, attr2: 0, pad: 0, };    // attr1 is 256-color  FIXME: move elsewhere

        entry.set_tile_id(tile_id);
        entry.set_x(x);
        entry.set_y(y);
        entry.set_is_8bpp(true);

        entry
    }

    pub fn get_y(&self) -> u8 {
        self.attr0 as u8
    }

    pub fn set_y(&mut self, y: u8) {
        self.attr0 = (self.attr0 & 0xff00) | (y as u16 & 0x00ff);
    }

    pub fn get_x(&self) -> u8 {
        self.attr1 as u8
    }

    pub fn set_x(&mut self, x: u16) {
        self.attr1 = (self.attr1 & 0xfe00) | (x & 0x01ff);
    }

    pub fn set_is_8bpp(&mut self, is_8bpp: bool) {
        self.attr0 = (self.attr0 & 0b1101111111111111) |
            if is_8bpp {
                0b0010000000000000
            } else {
                0b0000000000000000
            };
    }

    pub fn set_size(&mut self, size: u16) {
        self.attr1 = (self.attr1 & 0b0011111111111111) | size << 14;
    }

    pub fn set_tile_id(&mut self, id: u16) {
        self.attr2 = (self.attr2 & 0xfe00) | (id & 0x01ff);
    }

    pub fn oam_copy(&self) {
        unsafe {
            ptr::copy(&*self, gba::OAM_MEM as _, 1);
        };
    }
}
