pub trait Mem {
    fn mem_read(&mut self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
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
