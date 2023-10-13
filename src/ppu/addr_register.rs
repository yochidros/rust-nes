#[derive(Debug)]
pub struct AddrRegister {
    // first hi bytes, second lo bytes.
    value: (u8, u8),
    hi_ptr: bool,
}
const VRAM_MIN_ADDR: u16 = 0x0000;
const VRAM_MAX_ADDR: u16 = 0x3fff;

impl AddrRegister {
    pub fn new() -> Self {
        AddrRegister {
            value: (0, 0), // first hi bytes, second lo bytes.
            hi_ptr: true,
        }
    }
}

impl AddrRegister {
    fn set_addr(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }
    pub fn get_addr(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get_addr() > VRAM_MAX_ADDR {
            self.set_addr(self.get_addr() & 0x3FFF);
        }
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc_value: u8) {
        let old_lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc_value);
        if self.get_addr() > VRAM_MAX_ADDR {
            self.set_addr(self.get_addr() & 0x3FFF);
        }
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }
}
