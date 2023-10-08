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
