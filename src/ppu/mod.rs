pub mod addr_register;
pub mod control_register;
pub mod mask_register;

use addr_register::AddrRegister;

use self::{control_register::ControlRegister, mask_register::MaskRegister};

#[derive(Debug)]
pub struct NesPPU {
    // visiual of a game stored
    pub chr_rom: Vec<u8>,
    // keep palette tables used by a screen
    pub palette_table: [u8; 32],
    // 2kiB banks of spaces to hold background info
    pub vram: [u8; 2048],
    // keep state of sprites
    pub oam_data: [u8; 256],

    pub mirroring: PPUMirroring,
    pub control_reg: ControlRegister,
    pub mask_reg: MaskRegister,

    pub addr_reg: AddrRegister,
    internal_data_buf: u8,
}

#[derive(Debug, PartialEq)]
pub enum PPUMirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub trait PPU {
    fn write_to_control_reg(&mut self, value: u8);
    fn write_to_oam_data(&mut self, value: u8);

    fn write_to_ppu_addr(&mut self, value: u8);
    fn write_to_data(&mut self, value: u8);

    fn read_data(&mut self) -> u8;
    fn read_status(&mut self) -> u8;
    fn read_oam_data(&self) -> u8;
}

impl NesPPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: PPUMirroring) -> NesPPU {
        NesPPU {
            chr_rom,
            mirroring,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            control_reg: ControlRegister::new(),
            addr_reg: AddrRegister::new(),
            internal_data_buf: 0,
        }
    }
    fn increment_vram_addr(&mut self) {
        self.addr_reg
            .increment(self.control_reg.get_vram_addr_increment_value())
    }
}

impl PPU for NesPPU {
    fn write_to_control_reg(&mut self, value: u8) {
        self.control_reg.update(value)
    }

    fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr_reg.update(value);
    }
    fn write_to_data(&mut self, value: u8) {}

    fn write_to_oam_data(&mut self, value: u8) {
        todo!()
    }

    fn read_data(&mut self) -> u8 {
        let addr = self.addr_reg.get_addr();
        self.increment_vram_addr();
        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                // self.internal_data_buf = self.vram[self.mi];
                result
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {}",
                addr
            ),
            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unecpected access to mirrored space {}", addr),
        }
    }
    fn read_status(&mut self) -> u8 {
        0
    }
    fn read_oam_data(&self) -> u8 {
        0
    }
}

impl NesPPU {
    // 2000  2400
    // [ A ] [ B ]
    // 2800  2C00
    // [ C ] [ D ]

    // FourScreen:
    //   [ A ] [ B ]
    //   [ C ] [ D ]

    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]

    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    pub fn get_mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b1011_1111_1111_1111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // convert to vram vector
        let name_table_index = vram_index / 0x400; // convert to the name table index
        match (&self.mirroring, name_table_index) {
            (PPUMirroring::Vertical, 2) | (PPUMirroring::Vertical, 3) => vram_index - 0x800,
            (PPUMirroring::Horizontal, 1) | (PPUMirroring::Horizontal, 2) => vram_index - 0x400,
            (PPUMirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }
}
