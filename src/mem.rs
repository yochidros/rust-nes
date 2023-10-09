use crate::bus::Bus;

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo_bits = self.mem_read(pos) as u16;
        let hi_bits = self.mem_read(pos + 1) as u16;
        (hi_bits << 8) | lo_bits
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi_bits = (data >> 8) as u8;
        let lo_bits = (data & 0xff) as u8;
        self.mem_write(pos, lo_bits);
        self.mem_write(pos + 1, hi_bits);
    }
}
const RAM: u16 = 0x0000;
const RAM_MIRROS_END: u16 = 0x1fff;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRROR_END: u16 = 0x3fff;

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRROS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRROR_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!()
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("ignoring mem access at {}", addr);
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
            PPU_REGISTERS..=PPU_REGISTERS_MIRROR_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!()
            }
            0x8000..=0xFFFF => {
                panic!("attempt to write to cartridge rom space");
            }
            _ => {
                println!("ignoring mem write access at {}", addr);
            }
        }
    }
}
pub trait Stack {
    fn stack_pop(&mut self) -> u8;
    fn stack_push(&mut self, data: u8);

    fn stack_pop_u16(&mut self) -> u16 {
        let lo_bits = self.stack_pop() as u16;
        let hi_bits = self.stack_pop() as u16;
        (hi_bits << 8) | lo_bits
    }
    fn stack_push_u16(&mut self, data: u16) {
        let hi_bits = (data >> 8) as u8;
        let lo_bits = (data & 0xff) as u8;
        self.stack_push(hi_bits);
        self.stack_push(lo_bits)
    }
}
