use crate::opscodes::*;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,

    memory: [u8; 0xffff],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xffff],
        }
    }

    // pub fn load_and_run(&mut self, program: Vec<u8>) {
    //     self.program_counter = 0;
    //     loop {
    //         let ops_code = program[self.program_counter as usize];
    //         self.program_counter += 1;
    //
    //         match ops_code {
    //             0x00 => {
    //                 // BRK break
    //                 return;
    //             }
    //             0xA9 => {
    //                 // LDA (immediate) load data
    //                 let param = program[self.program_counter as usize];
    //                 self.lda(param);
    //                 self.program_counter += 1;
    //                 self.update_zero_and_negative_flags(self.register_a);
    //             }
    //             0xAA => {
    //                 // TAX  transfer Accumulator to X
    //                 self.tax(self.register_a);
    //                 self.update_zero_and_negative_flags(self.register_x);
    //             }
    //             0xe8 => {
    //                 // Increment X Register
    //                 self.inx();
    //                 self.update_zero_and_negative_flags(self.register_x);
    //             }
    //             _ => todo!(),
    //         }
    //     }
    // }
    //
    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::NonAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
    // memory
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

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

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        // launch or inserting new cartridge, then reset program ROM address state.
        self.load(program);
        self.reset();
        self.run();
    }
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        // 0xFFFC: program counter address set start point.
        self.mem_write_u16(0xFFFC, 0x8000);
    }
    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;
            let opcode = OPCODES_MAP.get(&code).expect("opcode not found");

            match code {
                // LDA
                0xA9 | 0xa5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0xAA => {
                    self.tax();
                    self.update_zero_and_negative_flags(self.register_x);
                }
                0xe8 => {
                    self.inx();
                    self.update_zero_and_negative_flags(self.register_x);
                }
                /* STA */
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }
                /* ADC */
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                // BRK break
                0x00 => return,
                _ => todo!(),
            }
            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.bytes_len - 1) as u16;
            }
        }
    }
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }
    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr); // M
        println!("address: {:x} value: {}", addr, value);
        let a = self.register_a as u16; // A
        let sum = a + value as u16;
        let has_carry = (self.status & 0b0000_0001) == 1;
        println!("has_carry: {}", has_carry);
        let sum_with_carry = if has_carry { sum + 1 } else { sum + 0 };
        let is_overflow = sum_with_carry > 0xff;
        println!("is_overflow: {}", is_overflow);
        println!("value: {:b}", sum_with_carry);
        if is_overflow {
            // set carry flag
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & !0b0000_0001;
        }
        let sum_with_carry_u8 = sum_with_carry as u8;
        // ^ XOR check is negative value (0x80: is Negative)
        // ex: 1000_0001 ^ 0110_0010 => 1110_0011 & 1000_000 => 1000_0000
        let is_negative = (sum_with_carry_u8 ^ self.register_a) & 0x80 != 0;
        println!("is_negative: {}", is_negative);
        if is_negative {
            // set overflow flag
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & !0b0100_0000;
        }
        self.register_a = sum_with_carry as u8;
    }

    // NVRB_DIZC (R 予約済み　使用できない)
    // N: negative
    // V: overflow
    // R: reserved
    // B: break command
    // D: decimal mode
    // I: interrupt diable
    // Z: zero flag set if value == 0
    // C: Carry
    fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.status = if value == 0 {
            self.status | 0b0000_0010
        } else {
            self.status & 0b1111_1101
        };
        if value & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0x9a_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.load_and_run(vec![0xa9, 0x0a, 0xAA, 0x00]);
        assert_eq!(cpu.register_x, 10);
    }
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1);
    }
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_adc_from_memory() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.register_a = 0x01;
        cpu.mem_write(0x8001, 0x01);
        cpu.run();
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.register_a, 0x2);
    }
    #[test]
    fn test_adc_from_memory_with_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.status = 0b0000_0001;
        cpu.register_a = 0x01;
        cpu.mem_write(0x8001, 0x01);
        cpu.run();
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.register_a, 0x3);
    }

    #[test]
    fn test_adc_from_memory_should_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.register_a = 0x01;
        cpu.mem_write(0x8001, 0xff);
        cpu.run();
        // is carry flag is true
        // 0000_0000 ^ 0000_0001 = 0000_0001 & 1000_0000
        assert_eq!(cpu.status & 0b0000_0001, 1);
        assert_eq!(cpu.register_a, 0x0);
    }

    #[test]
    fn test_adc_from_memory_with_has_carry_should_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.register_a = 0x7f;
        cpu.status = 0b0000_0001;
        cpu.mem_write(0x8001, 0x7f);
        cpu.run();
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.status & 0b0100_0000, 0x40);
        assert_eq!(cpu.register_a, 0xff);
    }

    #[test]
    fn test_adc_from_memory_with_plus() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.register_a = 0x50;
        cpu.status = 0b0000_0000;
        cpu.mem_write(0x8001, 0x90);
        cpu.run();
        assert_eq!(cpu.register_a, 0xe0);
    }

    #[test]
    fn test_adc_from_memory_with_minus() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.register_a = 0b0101_0000; //80
        cpu.status = 0b0000_0000;
        cpu.mem_write(0x8001, 0b1111_0000); // -112
        cpu.run();
        // 0101_0000 + 1111_0000 = 1_0100_0000 => 0100_0000 cast as u8
        // occur overflow not negative value
        assert_eq!(cpu.status & 0b0000_0001, 1);
        assert_eq!(cpu.status & 0b0100_0000, 0x0);
        assert_eq!(cpu.register_a, 0x40);
    }
}
