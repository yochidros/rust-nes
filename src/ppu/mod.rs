pub mod register;

use register::{
    addr_register::AddrRegister, control_register::ControlRegister, mask_register::MaskRegister,
    scroll_register::ScrollRegister, status_register::StatusRegister,
};
#[derive(Debug)]
pub struct NesPPU {
    // visiual of a game stored
    pub chr_rom: Vec<u8>,
    // keep palette tables used by a screen
    pub palette_table: [u8; 32],
    // 2kiB banks of spaces to hold background info
    pub vram: [u8; 2048],
    // keep state of sprites
    // https://www.nesdev.org/wiki/PPU_OAM
    pub oam_data: [u8; 256],
    pub oam_addr: u8,

    pub mirroring: PPUMirroring,
    pub control_reg: ControlRegister,
    pub mask_reg: MaskRegister,
    pub status_reg: StatusRegister,
    pub scroll_register: ScrollRegister,

    pub addr_reg: AddrRegister,
    pub cycles: usize,
    pub scanlines: u16,
    internal_data_buf: u8,
    pub nmi_interrupt: Option<u8>,
}

#[derive(Debug, PartialEq)]
pub enum PPUMirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub trait PPU {
    fn write_to_control_reg(&mut self, value: u8);
    fn write_to_mask_reg(&mut self, value: u8);

    fn write_to_oam_data(&mut self, value: u8);
    fn write_to_oam_addr(&mut self, value: u8);

    fn write_to_scroll_reg(&mut self, value: u8);
    fn write_to_ppu_addr(&mut self, value: u8);
    fn write_to_data(&mut self, value: u8);

    fn read_data(&mut self) -> u8;
    fn read_status(&mut self) -> u8;
    fn read_oam_data(&self) -> u8;

    fn write_to_oam_dma(&mut self, value: &[u8; 256]);
}

impl NesPPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: PPUMirroring) -> NesPPU {
        NesPPU {
            chr_rom,
            mirroring,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            oam_addr: 0,
            control_reg: ControlRegister::new(),
            addr_reg: AddrRegister::new(),
            mask_reg: MaskRegister::new(),
            status_reg: StatusRegister::new(),
            scroll_register: ScrollRegister::new(),
            scanlines: 0,
            cycles: 0,
            internal_data_buf: 0,
            nmi_interrupt: None,
        }
    }
    fn increment_vram_addr(&mut self) {
        self.addr_reg
            .increment(self.control_reg.get_vram_addr_increment_value())
    }
    // if scanaline over vblank, then start vblank interrupt
    /// return is reset scanlines
    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;
        // 341 ppu cycles per scan line.
        if self.cycles >= PPU_CYCLE_PER_SCAN_LINE {
            self.cycles = self.cycles - PPU_CYCLE_PER_SCAN_LINE;
            self.scanlines += 1;
            if self.scanlines == PPU_START_VBLANK {
                println!("start vblank");
                self.status_reg.update_vertical_blank_started(true);
                self.status_reg.update_sprite_0_hit(false);
                if self.control_reg.is_generate_vblank_nmi_on() {
                    self.nmi_interrupt = Some(1);
                }
            }
            if self.scanlines >= PPU_MAX_SCANLINE {
                println!("reset scanlines");
                self.scanlines = 0;
                self.nmi_interrupt = None;
                self.status_reg.update_sprite_0_hit(false);
                self.status_reg.reset_vblank_status();
                return true;
            }
        }
        return false;
    }
}
const PPU_CYCLE_PER_SCAN_LINE: usize = 341;
const PPU_START_VBLANK: u16 = 241;
const PPU_MAX_SCANLINE: u16 = 262;

impl PPU for NesPPU {
    fn write_to_control_reg(&mut self, value: u8) {
        // 1 or 32
        let before_nmi_status = self.control_reg.is_generate_vblank_nmi_on();
        self.control_reg.update(value);
        // bfore off, after on & is in vblank mode.
        if !before_nmi_status
            && self.control_reg.is_generate_vblank_nmi_on()
            && self.status_reg.is_in_vblank()
        {
            self.nmi_interrupt = Some(1);
        }
    }

    fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr_reg.update(value);
    }
    fn write_to_data(&mut self, value: u8) {
        let addr = self.addr_reg.get_addr();
        match addr {
            0..=0x1fff => {
                println!("attempt to write to chr rom space {}", addr);
            }
            0x2000..=0x2fff => {
                self.vram[self.get_mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {}",
                addr
            ),
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let mirrored_addr = addr - 0x10;
                self.palette_table[(mirrored_addr - 0x3f00) as usize] = value;
            }
            0x3f00..=0x3fff => {
                let mirrored_addr = addr - 0x3f00;
                println!(
                    "palette_table {:?} {}",
                    self.palette_table,
                    self.palette_table.len()
                );
                println!("mirrored_addr {:x}", mirrored_addr as usize);
                self.palette_table[(addr - 0x3f00) as usize] = value;
            }
            _ => panic!("unecpected access to mirrored space {}", addr),
        }
        self.increment_vram_addr();
    }

    fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
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
                self.internal_data_buf = self.vram[self.get_mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {}",
                addr
            ),
            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }
            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unecpected access to mirrored space {}", addr),
        }
    }

    fn read_status(&mut self) -> u8 {
        let value = self.status_reg.snapshot();
        self.status_reg.reset_vblank_status();
        self.addr_reg.reset_latch();
        self.scroll_register.reset_latch();
        value
    }

    fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    fn write_to_mask_reg(&mut self, value: u8) {
        self.mask_reg.update(value);
    }

    fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    fn write_to_scroll_reg(&mut self, value: u8) {
        self.scroll_register.write(value);
    }

    // to fully initialize the OAM by writing OAMDATA 256 times
    // using successive bytes from starting at address $100*N).
    fn write_to_oam_dma(&mut self, value: &[u8; 256]) {
        for x in value.iter() {
            self.oam_data[self.oam_addr as usize] = *x;
            self.oam_addr = self.oam_addr.wrapping_add(1);
        }
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
        let mirrored_vram = addr & 0b0010_1111_1111_1111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // convert to vram vector
        let name_table_index = vram_index / 0x400; // convert to the name table index
        match (&self.mirroring, name_table_index) {
            (PPUMirroring::Vertical, 2) | (PPUMirroring::Vertical, 3) => vram_index - 0x800,
            (PPUMirroring::Horizontal, 1) | (PPUMirroring::Horizontal, 2) => vram_index - 0x400,
            (PPUMirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }
    pub fn new_empty_rom() -> Self {
        NesPPU::new(vec![0; 2048], PPUMirroring::Horizontal)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_ppu_vram_writes() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.write_to_ppu_addr(0x23);
        ppu.write_to_ppu_addr(0x05);
        ppu.write_to_data(0x66);
        assert_eq!(ppu.vram[0x0305], 0x66);
    }
}
