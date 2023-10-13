use bitflags::*;

bitflags! {
   // 7  bit  0
   // ---- ----
   // VPHB SINN
   // |||| ||||
   // |||| ||++- Base nametable address
   // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
   // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
   // |||| |     (0: add 1, going across; 1: add 32, going down)
   // |||| +---- Sprite pattern table address for 8x8 sprites
   // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
   // |||+------ Background pattern table address (0: $0000; 1: $1000)
   // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
   // |+-------- PPU master/slave select
   // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
   // +--------- Generate an NMI at the start of the
   //            vertical blanking interval (0: off; 1: on)
    #[derive(Debug)]
    pub struct ControlRegister: u8 {
        const NAME_TABLE_1 = 0b00000001;
        const NAME_TABLE_2 = 0b00000010;
        const VRAM_ADD_INCREMENT = 0b00000100;
        const SPRITE_PATTERN_ADDR = 0b00001000;
        const BACKGROUND_PATTERN_ADDR = 0b00010000;
        const SPRITE_SIZE = 0b0010_0000;
        const MASTER_SLAVE_SELECT = 0b0100_0000;
        const GENERATE_NMI = 0b1000_0000;
    }
}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0)
    }
    pub fn nametable_addr(&self) -> u16 {
        match self.bits() & 0b11 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("unexpected nametable addr"),
        }
    }

    pub fn get_vram_addr_increment_value(&self) -> u8 {
        if !self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            1
        } else {
            32
        }
    }

    pub fn update(&mut self, data: u8) {
        *self = ControlRegister::from_bits_truncate(data)
    }

    pub fn is_generate_vblank_nmi_on(&self) -> bool {
        self.contains(ControlRegister::GENERATE_NMI)
    }

    pub fn master_slae_select(&self) -> u8 {
        if self.contains(ControlRegister::MASTER_SLAVE_SELECT) {
            1
        } else {
            0
        }
    }
    pub fn sprite_size(&self) -> u8 {
        if self.contains(ControlRegister::SPRITE_SIZE) {
            16
        } else {
            8
        }
    }

    pub fn background_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::BACKGROUND_PATTERN_ADDR) {
            0x1000
        } else {
            0
        }
    }
    pub fn sprite_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
            0x1000
        } else {
            0
        }
    }
}
