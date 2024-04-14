use bitflags::Flags;

use crate::cartridge;
use crate::cartridge::{mem::*, rom::Mirroring, rom::ROM};
use crate::joypad::Joypad;
use crate::ppu::{NesPPU, PPUMirroring, PPU};
use crate::rendering::frame::Frame;

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

pub struct Bus<'call> {
    // 2kib
    pub cpu_vram: [u8; 2048],
    pub rom: ROM,
    pub ppu: NesPPU,
    pub joypad: Joypad,
    pub frame: Frame,

    pub cycles: usize,
    gameloop_callback: Box<dyn FnMut(&NesPPU, &mut Joypad, &Frame) + 'call>,
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

impl<'a> Bus<'a> {
    pub fn new<'call, F>(rom: ROM, gameloop_callback: F) -> Bus<'call>
    where
        F: FnMut(&NesPPU, &mut Joypad, &Frame) + 'call,
    {
        let ppu = NesPPU::new(rom.chr_rom.clone(), rom.to_PPUMirroring());
        Bus {
            cpu_vram: [0; 2048],
            rom,
            ppu,
            joypad: Joypad::new(),
            frame: Frame::new(),
            cycles: 0,
            gameloop_callback: Box::from(gameloop_callback),
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
impl<'a> Bus<'a> {
    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        // ppu cycles 3x faster than cpu
        let nmi_before = self.ppu.nmi_interrupt.is_some();
        self.ppu.tick(cycles * 3, &mut self.frame);
        let nmi_after = self.ppu.nmi_interrupt.is_some();

        if !nmi_before && nmi_after {
            (self.gameloop_callback)(&self.ppu, &mut self.joypad, &self.frame);
        }
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.nmi_interrupt.take()
    }
}

// const RAM: u16 = 0x0000;
const RAM_MIRROS_END: u16 = 0x1fff;
const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_MIRROR_START: u16 = 0x2008;
const PPU_REGISTERS_MIRROR_END: u16 = 0x3fff;

impl Mem for Bus<'_> {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRROS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            0x2000 => self.ppu.control_reg.bits(),
            0x2001 => self.ppu.mask_reg.bits(),
            0x2003 | 0x2005 | 0x2006 | 0x4014 => 0,
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),
            PPU_REGISTERS_MIRROR_START..=PPU_REGISTERS_MIRROR_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_read(_mirror_down_addr)
            }
            0x4000..=0x4015 => 0, // apu
            0x4016 => self.joypad.read(),
            0x4017 => 0, // joypad 1 or 2
            0x6000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                panic!("ignoring mem access at {:x}", addr);
                0
            }
        }
    }
    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRROS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            0x2000 => self.ppu.write_to_control_reg(data),
            0x2001 => self.ppu.write_to_mask_reg(data),
            0x2002 => self.ppu.write_to_status_reg(data),
            0x2003 => self.ppu.write_to_oam_addr(data),
            0x2004 => self.ppu.write_to_oam_data(data),
            0x2005 => self.ppu.write_to_scroll_reg(data),
            0x2006 => self.ppu.write_to_ppu_addr(data),
            0x2007 => self.ppu.write_to_data(data),
            PPU_REGISTERS_MIRROR_START..=PPU_REGISTERS_MIRROR_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_write(_mirror_down_addr, data);
            }
            0x4000..=0x4013 | 0x4015 => {} // apu
            0x4016 => self.joypad.write(data),
            0x4017 => {} // joypad 1 or 2
            // https://wiki.nesdev.com/w/index.php/PPU_programmer_reference#OAM_DMA_.28.244014.29_.3E_write
            0x4014 => {
                let mut buffer: [u8; 256] = [0; 256];
                let hi: u16 = (data as u16) << 8;
                for i in 0..256u16 {
                    buffer[i as usize] = self.mem_read(hi + i);
                }
                self.ppu.write_to_oam_dma(&buffer);
            }
            0x4018..=0xFFFF => {
                panic!("attempt to write to cartridge rom space, {:x}", addr);
            }
            _ => {
                panic!("ignoring mem write access at {}", addr);
            }
        }
    }
}
