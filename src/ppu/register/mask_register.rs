use bitflags::*;

bitflags! {
    /*
        7  bit  0
        ---- ----
        BGRs bMmG
        |||| ||||
        |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
        |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
        |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
        |||| +---- 1: Show background
        |||+------ 1: Show sprites
        ||+------- Emphasize red (green on PAL/Dendy)
        |+-------- Emphasize green (red on PAL/Dendy)
        +--------- Emphasize blue
   */
    #[derive(Debug)]
    pub struct MaskRegister: u8 {
        const GREYSCALE = 0b0000_0001;
        const SHOW_BACKGROUND_IN_LEFTMOST_8PX = 0b0000_0010;
        const SHOW_SPRITES_IN_LEFTMOST_8PX = 0b0000_0100;
        const SHOW_BACKGROUND = 0b0000_1000;
        const SHOW_SPRITES = 0b0001_0000;
        const EMPHASIZE_RED = 0b0010_0000;
        const EMPHASIZE_GREEN = 0b0100_0000;
        const EMPHASIZE_BLUE = 0b1000_0000;
    }
}
pub enum Color {
    Red,
    Green,
    Blue,
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister::from_bits_truncate(0)
    }
    pub fn update(&mut self, data: u8) {
        *self = MaskRegister::from_bits_truncate(data)
    }

    pub fn grayscale(&self) -> bool {
        self.contains(MaskRegister::GREYSCALE)
    }
    pub fn show_background_in_leftmost_8px(&self) -> bool {
        self.contains(MaskRegister::SHOW_BACKGROUND_IN_LEFTMOST_8PX)
    }
    pub fn show_sprites_in_leftmost_8px(&self) -> bool {
        self.contains(MaskRegister::SHOW_SPRITES_IN_LEFTMOST_8PX)
    }
    pub fn show_background(&self) -> bool {
        self.contains(MaskRegister::SHOW_BACKGROUND)
    }
    pub fn show_sprites(&self) -> bool {
        self.contains(MaskRegister::SHOW_SPRITES)
    }

    pub fn emphasize(&self) -> Vec<Color> {
        let mut result = Vec::<Color>::new();
        if self.contains(MaskRegister::EMPHASIZE_RED) {
            result.push(Color::Red);
        }
        if self.contains(MaskRegister::EMPHASIZE_GREEN) {
            result.push(Color::Green);
        }
        if self.contains(MaskRegister::EMPHASIZE_BLUE) {
            result.push(Color::Green);
        }
        result
    }
}
