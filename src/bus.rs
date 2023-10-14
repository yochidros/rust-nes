use crate::cartridge::{Mirroring, ROM};
use crate::ppu::{NesPPU, PPUMirroring, PPU};

//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | I/O Registers |       |               |
// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|

const RAM: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_REG: u16 = 0x2000;
const PPU_REG_END: u16 = 0x3FFF;

#[derive(Debug)]
pub struct Bus {
    // 2kib
    pub cpu_vram: [u8; 2048],
    pub rom: ROM,
    pub ppu: NesPPU,

    pub cycles: usize,
}

impl ROM {
    fn to_PPUMirroring(&self) -> PPUMirroring {
        match self.screen_mirroring {
            Mirroring::Horizontal => PPUMirroring::Horizontal,
            Mirroring::Vertical => PPUMirroring::Vertical,
            Mirroring::FourScreen => PPUMirroring::FourScreen,
        }
    }
}

impl Bus {
    pub fn new(rom: ROM) -> Self {
        let ppu = NesPPU::new(rom.chr_rom.clone(), rom.to_PPUMirroring());
        Bus {
            cpu_vram: [0; 2048],
            rom,
            ppu,
            cycles: 0,
        }
    }
    pub fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}
impl Bus {
    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        // ppu cycles 3x faster than cpu
        self.ppu.tick(cycles * 3);
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.nmi_interrupt.take()
    }
}
impl Bus {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", addr)
            }
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),

            0x2008..=PPU_REG_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_read(mirror_down_addr)
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("Invalid memory address {:x}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            0x2000 => {
                self.ppu.write_to_control_reg(data);
            }
            0x2006 => {
                self.ppu.write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.write_to_data(data);
            }
            0x2008..=PPU_REG_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_write(mirror_down_addr, data);
            }
            0x8000..=0xFFFF => {
                panic!("Attempt to write to ROM address {:x}", addr);
            }
            _ => {
                println!("Invalid memory address {:x}", addr);
            }
        }
    }
}
