use crate::cartridge::ROM;
#[derive(Debug)]
pub struct Bus {
    // 2kib
    pub cpu_vram: [u8; 2048],
    pub rom: ROM,
}

impl Bus {
    pub fn new(rom: ROM) -> Self {
        Bus {
            cpu_vram: [0; 2048],
            rom,
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
