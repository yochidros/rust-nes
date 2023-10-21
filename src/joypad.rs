use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone)]
    pub struct JoypadButton: u8 {
        const BUTTON_A    = 0b00000001;
        const BUTTON_B     = 0b00000010;
        const SELECT     = 0b00000100;
        const START       = 0b00001000;
        const UP    = 0b00010000;
        const DOWN   = 0b00100000;
        const LEFT = 0b01000000;
        const RIGHT = 0b10000000;
    }
}
pub struct Joypad {
    pub button_status: JoypadButton,
    pub strobe: bool,
    pub index: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            button_status: JoypadButton::from_bits_truncate(0),
            strobe: false,
            index: 0,
        }
    }

    pub fn write(&mut self, data: u8) {
        self.strobe = data & 0x1 == 1;
        if self.strobe {
            self.index = 0;
        }
    }

    pub fn read(&mut self) -> u8 {
        if self.index > 7 {
            return 1;
        }
        let response = (self.button_status.bits() & (1 << self.index)) >> self.index;
        if !self.strobe && self.index <= 7 {
            self.index += 1;
        }
        response
    }
    pub fn set_button_pressed_status(&mut self, button: JoypadButton, pressed: bool) {
        self.button_status.set(button, pressed);
    }
}
