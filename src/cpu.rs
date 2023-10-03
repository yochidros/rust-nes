pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;
        loop {
            let ops_code = program[self.program_counter as usize];
            self.program_counter += 1;

            match ops_code {
                0x00 => {
                    // BRK break
                    return;
                }
                0xA9 => {
                    // LDA (immediate) load data
                    let param = program[self.program_counter as usize];
                    self.lda(param);
                    self.program_counter += 1;
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0xAA => {
                    // TAX  transfer Accumulator to X
                    self.tax(self.register_a);
                    self.update_zero_and_negative_flags(self.register_x);
                }
                0xe8 => {
                    // Increment X Register
                    self.inx();
                    self.update_zero_and_negative_flags(self.register_x);
                }
                _ => todo!(),
            }
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
    }

    fn tax(&mut self, value: u8) {
        self.register_x = value;
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
    }

    // NVRB_DIZC (R 予約済み　使用できない)
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
        cpu.interpret(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xAA, 0x00]);
        assert_eq!(cpu.register_x, 10);
    }
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1);
    }
    #[test]
    fn test_0xe8_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 1);
    }
}
