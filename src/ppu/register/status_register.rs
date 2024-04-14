use bitflags::bitflags;

bitflags! {
    /*
    7  bit  0
    ---- ----
    VSO. ....
    |||| ||||
    |||+-++++- PPU open bus. Returns stale PPU bus contents.
    ||+------- Sprite overflow. The intent was for this flag to be set
    ||         whenever more than eight sprites appear on a scanline, but a
    ||         hardware bug causes the actual behavior to be more complicated
    ||         and generate false positives as well as false negatives; see
    ||         PPU sprite evaluation. This flag is set during sprite
    ||         evaluation and cleared at dot 1 (the second dot) of the
    ||         pre-render line.
    |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
    |          a nonzero background pixel; cleared at dot 1 of the pre-render
    |          line.  Used for raster timing.
    +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
               Set at dot 1 of line 241 (the line *after* the post-render
               line); cleared after reading $2002 and at dot 1 of the
               pre-render line.
    */
    #[derive(Debug)]
    pub struct StatusRegister: u8 {
        const PPU_OPEN_BUS = 0b0001_1111;
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_0_HIT = 0b0100_0000;
        const VERTICAL_BLANK_STARTED = 0b1000_0000;
    }

}
impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister::from_bits_truncate(0)
    }
    pub fn update(&mut self, value: u8) {
        *self = StatusRegister::from_bits_truncate(value);
    }

    pub fn update_sprite_overflow(&mut self, value: bool) {
        self.set(StatusRegister::SPRITE_OVERFLOW, value);
    }
    pub fn update_sprite_0_hit(&mut self, value: bool) {
        self.set(StatusRegister::SPRITE_0_HIT, value);
    }
    pub fn is_sprite_0_hit(&self) -> bool {
        self.contains(StatusRegister::SPRITE_0_HIT)
    }

    pub fn update_vertical_blank_started(&mut self, value: bool) {
        self.set(StatusRegister::VERTICAL_BLANK_STARTED, value);
    }
    pub fn is_in_vblank(&self) -> bool {
        self.contains(StatusRegister::VERTICAL_BLANK_STARTED)
    }
    pub fn reset_vblank_status(&mut self) {
        self.remove(StatusRegister::VERTICAL_BLANK_STARTED);
    }
    pub fn snapshot(&self) -> u8 {
        self.bits()
    }
}
