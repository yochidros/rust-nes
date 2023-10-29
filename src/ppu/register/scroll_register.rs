use std::backtrace::Backtrace;

#[derive(Debug)]
pub struct ScrollRegister {
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub latch: bool,
    pub fine_scroll_x: u8,
    pub fine_scroll_y: u8,
}
impl ScrollRegister {
    pub fn new() -> Self {
        ScrollRegister {
            scroll_x: 0,
            scroll_y: 0,
            latch: false,
            fine_scroll_x: 0,
            fine_scroll_y: 0,
        }
    }

    pub fn write(&mut self, data: u8) {
        if self.latch {
            self.scroll_y = data;
            self.fine_scroll_y = data & 0x07;
        } else {
            self.scroll_x = data;
            self.fine_scroll_x = data & 0x07;
        }
        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        // println!("resetting latch");
        self.latch = false;
    }
}
