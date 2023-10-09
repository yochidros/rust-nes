use crate::flags::*;
use crate::mem::*;
use crate::opscodes::*;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: StatusFlags,
    pub program_counter: u16,
    pub stack_pointer: u8,

    memory: [u8; 0xffff],
}

const STACK_BASE: u16 = 0x0100;

impl CPU {
    #[allow(dead_code)]
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: StatusFlags::from_bits_truncate(0b100100),
            program_counter: 0,
            stack_pointer: 0,
            memory: [0; 0xffff],
        }
    }
}
impl AddressingModeConverter for CPU {
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
}

// Memory
impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
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
                // ASL with Accumulator
                0x0a => {
                    self.asl_accumulator();
                    self.update_zero_and_negative_flags(self.register_a);
                }
                // ASL with memory
                0x06 | 0x16 | 0x0e | 0x1e => {
                    let value = self.asl(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                    self.update_zero_and_negative_flags(value);
                }
                // LSR with Accumulator
                0x4a => {
                    self.lsr_accumulator();
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0x46 | 0x56 | 0x4e | 0x5e => {
                    let value = self.lsr(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                    self.update_zero_and_negative_flags(value);
                }
                // ROR with Accumulator
                0x6a => {
                    self.ror_accumulator();
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0x66 | 0x76 | 0x6e | 0x7e => {
                    let value = self.ror(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                    self.update_zero_and_negative_flags(value);
                }
                // ROL with Accumulator
                0x2a => {
                    self.rol_accumulator();
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0x26 | 0x36 | 0x2e | 0x3e => {
                    let value = self.rol(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                    self.update_zero_and_negative_flags(value);
                }
                // LDX
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.ldx(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_x);
                }
                // LDY
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    self.ldy(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_y);
                }
                // TAX
                0xAA => {
                    self.tax();
                    self.update_zero_and_negative_flags(self.register_x);
                }
                0x8A => {
                    self.txa();
                    self.update_zero_and_negative_flags(self.register_a);
                }
                // TAY
                0xA8 => {
                    self.tay();
                    self.update_zero_and_negative_flags(self.register_y);
                }
                // TYA
                0x98 => {
                    self.tya();
                    self.update_zero_and_negative_flags(self.register_a);
                }
                // INC
                0xe6 | 0xf6 | 0xee | 0xfe => {
                    self.inc(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0xe8 => {
                    self.inx();
                    self.update_zero_and_negative_flags(self.register_x);
                }
                0xc8 => {
                    self.iny();
                    self.update_zero_and_negative_flags(self.register_y);
                }
                // DEC
                0xc6 | 0xd6 | 0xce | 0xde => {
                    self.dec(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                // DEX
                0xca => {
                    self.dex();
                    self.update_zero_and_negative_flags(self.register_x);
                }
                // DEY
                0x88 => {
                    self.dey();
                    self.update_zero_and_negative_flags(self.register_y);
                }
                // BCS
                0xb0 => {
                    self.branch(self.status.contains(StatusFlags::CARRY));
                }
                // BCC
                0x90 => {
                    self.branch(!self.status.contains(StatusFlags::CARRY));
                }
                // BEQ
                0xf0 => {
                    self.branch(self.status.contains(StatusFlags::ZERO));
                }
                // BNE
                0xd0 => {
                    self.branch(!self.status.contains(StatusFlags::ZERO));
                }
                // BVS
                0x70 => {
                    self.branch(self.status.contains(StatusFlags::OVERFLOW));
                }
                // BVC
                0x50 => {
                    self.branch(!self.status.contains(StatusFlags::OVERFLOW));
                }
                // TSX
                0xba => {
                    self.tsx();
                    self.update_zero_and_negative_flags(self.register_x);
                }
                // PHA
                0x48 => {
                    self.pha();
                }
                // PLA
                0x68 => {
                    self.pla();
                    self.update_zero_and_negative_flags(self.register_a);
                }
                // PHP
                0x08 => {
                    self.php();
                }
                // PLP
                0x28 => {
                    self.plp();
                }
                // TXS
                0x9a => {
                    self.txs();
                }
                // JMP
                0x4c | 0x6c => {
                    self.jmp();
                }
                // JSR
                0x20 => {
                    self.jsr();
                }
                // RTS
                0x60 => {
                    self.rts();
                }
                // RTI
                0x40 => {
                    self.rti();
                }
                // CMP
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    self.cmp(&opcode.mode);
                }
                0xe0 | 0xe4 | 0xec => {
                    self.cpx(&opcode.mode);
                }
                0xc0 | 0xc4 | 0xcc => {
                    self.cpy(&opcode.mode);
                }
                /* STA */
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }
                /* STX */
                0x86 | 0x96 | 0x8e => {
                    self.stx(&opcode.mode);
                }
                /* STY */
                0x84 | 0x94 | 0x8c => {
                    self.sty(&opcode.mode);
                }
                /* ADC */
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                /* SBC */
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    self.sbc(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                /* AND */
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                /* EOR */
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                    self.eor(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                /* ORA */
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                    self.ora(&opcode.mode);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                /* BIT */
                0x24 | 0x2c => {
                    self.bit(&opcode.mode);
                }
                /* SEC */
                0x38 => {
                    self.sec();
                }
                /* CLC */
                0x18 => {
                    self.clc();
                }
                /* SED */
                0xf8 => {
                    self.sed();
                }
                /* CLD */
                0xd8 => {
                    self.cld();
                }
                /* SEI */
                0x78 => {
                    self.sei();
                }
                /* CLI */
                0x58 => {
                    self.cli();
                }
                /* CLV */
                0xb8 => {
                    self.clv();
                }
                /* NOP */
                0xea => {
                    self.nop();
                }
                /* BRK */
                0x00 => {
                    self.brk();
                    break;
                }
                _ => todo!(),
            }
            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.bytes_len - 1) as u16;
            }
        }
    }
}
impl Stack for CPU {
    fn stack_push(&mut self, data: u8) {
        self.mem_write((STACK_BASE as u16) + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1)
    }
    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read((STACK_BASE as u16) + self.stack_pointer as u16)
    }
}

impl CPU {
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = StatusFlags::from_bits_truncate(0b0010_0100);

        self.program_counter = self.mem_read_u16(0xFFFC);
    }
    fn asl_accumulator(&mut self) {
        let value = self.register_a;
        self.status = if value & StatusFlags::CARRY.bits() > 0 {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let result = value << 1;
        self.register_a = result;
    }

    fn asl(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.status = if value & StatusFlags::CARRY.bits() > 0 {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let result = value << 1;
        self.mem_write(addr, result);
        result
    }
    fn lsr_accumulator(&mut self) {
        let value = self.register_a;
        self.status = if value & StatusFlags::CARRY.bits() == 1 {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let result = value >> 1;
        self.register_a = result;
    }

    fn lsr(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.status = if value & StatusFlags::CARRY.bits() == 1 {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let result = value >> 1;
        self.mem_write(addr, result);
        result
    }
    fn ror_accumulator(&mut self) {
        let value = self.register_a;
        let current_carry = self.status.bits() & StatusFlags::CARRY.bits();
        let old_carry = value & StatusFlags::CARRY.bits();
        self.status = if old_carry > 0 {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let result = (value >> 1) | (current_carry << 7);
        self.register_a = result;
    }
    fn ror(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let current_carry = self.status.bits() & StatusFlags::CARRY.bits();
        let old_carry = value & StatusFlags::CARRY.bits();
        self.status = if old_carry > 0 {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let result = (value >> 1) | (current_carry << 7);
        self.mem_write(addr, result);
        result
    }
    fn rol_accumulator(&mut self) {
        let value = self.register_a;
        let current_carry = self.status.bits() & StatusFlags::CARRY.bits();
        let old_carry = value >> 7;
        self.status = if old_carry > 0 {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let result = (value << 1) | current_carry;
        self.register_a = result;
    }
    fn rol(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let current_carry = self.status.bits() & StatusFlags::CARRY.bits();
        let old_carry = value >> 7;
        self.status = if old_carry > 0 {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let result = (value << 1) | current_carry;
        self.mem_write(addr, result);
        result
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_x = value;
    }
    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_y = value;
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
    }
    fn txa(&mut self) {
        self.register_a = self.register_x;
    }
    fn tay(&mut self) {
        self.register_y = self.register_a;
    }
    fn tya(&mut self) {
        self.register_a = self.register_y;
    }

    fn jmp(&mut self) {
        let addr = self.mem_read_u16(self.program_counter);
        self.program_counter = addr;
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.mem_write(addr, value.wrapping_add(1));
    }
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
    }
    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
    }
    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.mem_write(addr, value.wrapping_sub(1));
    }
    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
    }
    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
    }
    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
    }
    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = self.register_a.wrapping_sub(value);
        self.status = if self.register_a >= value {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        self.status = if self.register_a == value {
            self.status | StatusFlags::ZERO
        } else {
            self.status & !StatusFlags::ZERO
        };
        self.status = if result & StatusFlags::NEGATIVE.bits() > 0 {
            self.status | StatusFlags::NEGATIVE
        } else {
            self.status & !StatusFlags::NEGATIVE
        };
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = self.register_x.wrapping_sub(value);
        self.status = if self.register_x >= value {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        self.status = if self.register_x == value {
            self.status | StatusFlags::ZERO
        } else {
            self.status & !StatusFlags::ZERO
        };
        self.status = if result & StatusFlags::NEGATIVE.bits() > 0 {
            self.status | StatusFlags::NEGATIVE
        } else {
            self.status & !StatusFlags::NEGATIVE
        };
    }
    fn cpy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = self.register_y.wrapping_sub(value);
        self.status = if self.register_y >= value {
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        self.status = if self.register_y == value {
            self.status | StatusFlags::ZERO
        } else {
            self.status & !StatusFlags::ZERO
        };
        self.status = if result & StatusFlags::NEGATIVE.bits() > 0 {
            self.status | StatusFlags::NEGATIVE
        } else {
            self.status & !StatusFlags::NEGATIVE
        };
    }
    fn branch(&mut self, condition: bool) {
        if condition {
            let offset = self.mem_read(self.program_counter) as i8;
            let jump_addr = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(offset as u16);
            self.program_counter = jump_addr;
        }
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }
    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }
    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }
    fn jsr(&mut self) {
        self.stack_push_u16(self.program_counter + 2 - 1);
        let target_address = self.mem_read_u16(self.program_counter);
        self.program_counter = target_address
    }
    fn rts(&mut self) {
        let target_address = self.stack_pop_u16();
        self.program_counter = target_address + 1;
    }
    fn rti(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.stack_pop());
        self.program_counter = self.stack_pop_u16()
    }
    fn pha(&mut self) {
        let a = self.register_a;
        self.stack_push(a);
    }
    fn pla(&mut self) {
        self.register_a = self.stack_pop();
    }
    fn php(&mut self) {
        let mut value = self.status.bits();
        value = value | StatusFlags::BREAK.bits();
        value = value | StatusFlags::RESERVED.bits();
        self.stack_push(value);
    }
    fn plp(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.stack_pop());
        self.status = self.status & !StatusFlags::BREAK;
        self.status = self.status | StatusFlags::RESERVED;
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr); // M
        let a = self.register_a as u16; // A
        let sum = a + value as u16;
        let has_carry = self.status.contains(StatusFlags::CARRY);
        let sum_with_carry = if has_carry { sum + 1 } else { sum + 0 };
        let is_overflow = sum_with_carry > 0xff;
        self.status = if is_overflow {
            // set carry flag
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let sum_with_carry_u8 = sum_with_carry as u8;
        // ^ XOR check is negative value (0x80: is Negative)
        // ex: 1000_0001 ^ 0110_0010 => 1110_0011 & 1000_000 => 1000_0000
        let is_negative =
            (value ^ sum_with_carry_u8) & (sum_with_carry_u8 ^ self.register_a) & 0x80 != 0;
        self.status = if is_negative {
            // set overflow flag
            self.status | StatusFlags::OVERFLOW
        } else {
            self.status & !StatusFlags::OVERFLOW
        };
        self.register_a = sum_with_carry as u8;
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr); // M
                                         // A - M - (1 - C)
                                         // = A - M - 1 + C
                                         // = A - (M + 1) + C
                                         // = A + (-M + -1) + C
        let a = self.register_a as u16; // A
        let b = (value as i8).wrapping_neg().wrapping_sub(1) as u8;
        let sum_with_carry = if self.status.contains(StatusFlags::CARRY) {
            a.wrapping_add(b as u16).wrapping_add(1)
        } else {
            a.wrapping_add(b as u16)
        };
        let is_overflow = sum_with_carry > 0xff;
        self.status = if is_overflow {
            // set carry flag
            self.status | StatusFlags::CARRY
        } else {
            self.status & !StatusFlags::CARRY
        };
        let sum_with_carry_u8 = sum_with_carry as u8;
        // ^ XOR check is negative value (0x80: is Negative)
        // ex: 0000_0001 ^ 1000_0001 = 1000_0000
        // 1000_0001 ^ 0110_0010 => 1110_0011 & 1000_000 => 1000_0000
        // 1000_0000 & 1000_0000 & 1000_0000
        let is_negative =
            (b as u8 ^ sum_with_carry_u8) & (sum_with_carry_u8 ^ self.register_a) & 0x80 != 0;
        self.status = if is_negative {
            // set overflow flag
            self.status | StatusFlags::OVERFLOW
        } else {
            self.status & !StatusFlags::OVERFLOW
        };
        self.register_a = sum_with_carry as u8;
    }
    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr); // M
        self.register_a = self.register_a & value;
    }
    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr); // M
        self.register_a = self.register_a ^ value;
    }
    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr); // M
        self.register_a = self.register_a | value;
    }
    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        println!("register_a: {:04x}", self.program_counter);
        println!("addr: {:04x}", addr);
        let value = self.mem_read(addr); // M
        let result = self.register_a & value;
        println!("{}, {} result: {:08b}", self.register_a, value, result);
        self.status = if result == 0 {
            self.status | StatusFlags::ZERO
        } else {
            self.status & !StatusFlags::ZERO
        };
        self.status = if value & StatusFlags::OVERFLOW.bits() > 0 {
            self.status | StatusFlags::OVERFLOW
        } else {
            self.status & !StatusFlags::OVERFLOW
        };
        self.status = if value & StatusFlags::NEGATIVE.bits() > 0 {
            self.status | StatusFlags::NEGATIVE
        } else {
            self.status & !StatusFlags::NEGATIVE
        };
    }

    fn brk(&mut self) {
        self.status = self.status | StatusFlags::BREAK;
    }
    fn sec(&mut self) {
        self.status = self.status | StatusFlags::CARRY;
    }
    fn clc(&mut self) {
        self.status = self.status & !StatusFlags::CARRY;
    }
    fn sed(&mut self) {
        self.status = self.status | StatusFlags::DECIMAL_MODE;
    }
    fn cld(&mut self) {
        self.status = self.status & !StatusFlags::DECIMAL_MODE;
    }
    fn sei(&mut self) {
        self.status = self.status | StatusFlags::INTERRUPT_DISABLE;
    }
    fn cli(&mut self) {
        self.status = self.status & !StatusFlags::INTERRUPT_DISABLE;
    }
    fn clv(&mut self) {
        self.status = self.status & !StatusFlags::OVERFLOW;
    }
    fn nop(&self) {}

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
            self.status | StatusFlags::ZERO
        } else {
            self.status & !StatusFlags::ZERO
        };
        self.status = if value & 0b1000_0000 != 0 {
            // replace self.status.insert(NEGATIVE);
            self.status | StatusFlags::NEGATIVE
        } else {
            self.status & !StatusFlags::NEGATIVE
        };
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
        assert_eq!(cpu.status.bits() & StatusFlags::ZERO.bits(), 0b00);
        assert!(cpu.status.bits() & StatusFlags::NEGATIVE.bits() == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
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
    fn test_inc() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe6, 0x01, 0x00]);
        cpu.reset();
        cpu.mem_write(0x01, 0x01);
        cpu.run();
        let value = cpu.mem_read(0x01);
        assert_eq!(value, 2);
    }
    #[test]
    fn test_iny() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xc8, 0x00]);
        cpu.reset();
        cpu.register_y = 0x01;
        cpu.run();
        assert_eq!(cpu.register_y, 2);
    }
    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }
    #[test]
    fn test_ldx() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA2, 0x10, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 0x10);
    }
    #[test]
    fn test_ldy() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa0, 0xff, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_y, 0xff);
    }

    #[test]
    fn test_adc_from_memory() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.register_a = 0x01;
        cpu.mem_write(0x8001, 0x01);
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
        assert_eq!(cpu.register_a, 0x2);
    }
    #[test]
    fn test_adc_from_memory_with_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.status = StatusFlags::CARRY;
        cpu.register_a = 0x01;
        cpu.mem_write(0x8001, 0x01);
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
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
        assert_eq!(cpu.status.bits() & 0b0000_0001, 1);
        assert_eq!(cpu.register_a, 0x0);
    }

    #[test]
    fn test_adc_from_memory_with_has_carry_should_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.register_a = 0x7f;
        cpu.status = StatusFlags::CARRY;
        cpu.mem_write(0x8001, 0x7f);
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
        assert_eq!(cpu.status.bits() & 0b0100_0000, 0x40);
        assert_eq!(cpu.register_a, 0xff);
    }

    #[test]
    fn test_adc_from_memory_with_plus() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x00]);
        cpu.reset();
        cpu.register_a = 0x50;
        cpu.status = StatusFlags::from_bits_truncate(0b00000000);
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
        cpu.status = StatusFlags::from_bits_truncate(0b00000000);
        cpu.mem_write(0x8001, 0b1111_0000); // -112
        cpu.run();
        // 0101_0000 + 1111_0000 = 1_0100_0000 => 0100_0000 cast as u8
        // occur overflow not negative value
        assert_eq!(cpu.status.bits() & 0b0000_0001, 1);
        assert_eq!(cpu.status.bits() & 0b0100_0000, 0x0);
        assert_eq!(cpu.register_a, 0x40);
    }
    #[test]
    fn test_sbc_from_memory() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x10, 0x00]);
        cpu.reset();
        cpu.register_a = 0x20;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 1);
        assert_eq!(cpu.register_a, 0x0f);
    }

    #[test]
    fn test_sbc_from_memory_with_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x10, 0x00]);
        cpu.reset();
        cpu.register_a = 0x20;
        cpu.status = StatusFlags::CARRY;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 1);
        assert_eq!(cpu.register_a, 0x10);
    }

    #[test]
    fn test_sbc_from_memory_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x00, 0x00]);
        cpu.reset();
        cpu.register_a = 0x0;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0000, 0);
        assert_eq!(cpu.register_a, 0xff);
    }
    #[test]
    fn test_sbc_from_memory_overflow_with_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x00, 0x00]);
        cpu.reset();
        cpu.register_a = 0x0;
        cpu.status = StatusFlags::CARRY;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0000, 0);
        assert_eq!(cpu.register_a, 0x0);
    }
    #[test]
    fn test_sbc_from_memory_with_minus() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0xf0, 0x00]); // decimal: -112
        cpu.reset();
        cpu.register_a = 0x00; // decimal: 80
                               // 0 - (70) = -70
        cpu.run();
        // 0101_0000 + 1111_0000 = 1_0100_0000 => 0100_0000 cast as u8
        // occur overflow not negative value
        assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
        assert_eq!(cpu.status.bits() & 0b0100_0000, 0x0);
        assert_eq!(cpu.register_a, 0x0f);
    }

    #[test]
    fn test_sbc_from_memory_with_minus_with_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0xf0, 0x00]); // decimal: -70
        cpu.reset();
        cpu.register_a = 0xf0; // decimal: -70
        cpu.status = StatusFlags::CARRY;
        cpu.run();
        // 0101_0000 + 1111_0000 = 1_0100_0000 => 0100_0000 cast as u8
        // occur overflow not negative value
        assert_eq!(cpu.status.bits() & 0b0000_0001, 1);
        assert_eq!(cpu.status.bits() & 0b0100_0000, 0x0);
        assert_eq!(cpu.register_a, 0x0);
    }
    #[test]
    fn test_sbc_from_memory_with_plus() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x20, 0x00]);
        cpu.reset();
        cpu.register_a = 0x10;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0x80);
        assert_eq!(cpu.register_a, 0xef);
    }
    #[test]
    fn test_sbc_from_memory_with_plus_with_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x20, 0x00]);
        cpu.reset();
        cpu.register_a = 0x10;
        cpu.status = StatusFlags::CARRY;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0x80);
        assert_eq!(cpu.register_a, 0xf0);
    }
    #[test]
    fn test_sbc_from_memory_with_plus_with_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x70, 0x00]);
        cpu.reset();
        cpu.register_a = 0x70;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0x80);
        assert_eq!(cpu.register_a, 0xff);
    }
    #[test]
    fn test_sbc_from_memory_with_plus_with_overflow_with_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x70, 0x00]);
        cpu.reset();
        cpu.register_a = 0x70;
        cpu.status = StatusFlags::CARRY;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 1);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0x0);
        assert_eq!(cpu.register_a, 0x00);
    }

    #[test]
    fn test_and_from_memory() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x29, 0x10, 0x00]);
        cpu.reset();
        cpu.register_a = 0x10;
        cpu.run();
        assert_eq!(cpu.register_a, 0x10);
    }
    #[test]
    fn test_eor_from_memory() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x49, 0x10, 0x00]);
        cpu.reset();
        cpu.register_a = 0x08;
        cpu.run();
        assert_eq!(cpu.register_a, 0x18);
    }
    #[test]
    fn test_ora_from_memory() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x09, 0x10, 0x00]);
        cpu.reset();
        cpu.register_a = 0x08;
        cpu.run();
        assert_eq!(cpu.register_a, 0x18);
    }
    #[test]
    fn test_bit_all_false() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x24, 0x01, 0x00]);
        cpu.reset();
        cpu.mem_write(0x01, 0x01);
        cpu.register_a = 0x01;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0010, 0);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0);
        assert_eq!(cpu.status.bits() & 0b0100_0000, 0);
    }
    #[test]
    fn test_bit_zero_true() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x24, 0x01, 0x00]);
        cpu.reset();
        cpu.mem_write(0x01, 0x00);
        cpu.register_a = 0x01;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0010, 0b10);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0);
        assert_eq!(cpu.status.bits() & 0b0100_0000, 0);
    }
    #[test]
    fn test_bit_zero_true_overflow_true() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x24, 0x01, 0x00]);
        cpu.reset();
        cpu.mem_write(0x01, 0x42);
        cpu.register_a = 0x01;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0010, 0b10);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0);
        assert_eq!(
            cpu.status.bits() & 0b0100_0000,
            StatusFlags::OVERFLOW.bits()
        );
    }
    #[test]
    fn test_bit_zero_true_overflow_true_negative_true() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x24, 0x01, 0x00]);
        cpu.reset();
        cpu.mem_write(0x01, 0xC2);
        cpu.register_a = 0x01;
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0010, 0b10);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0b1000_0000);
        assert_eq!(
            cpu.status.bits() & 0b0100_0000,
            StatusFlags::OVERFLOW.bits()
        );
    }
    #[test]
    fn test_brk() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0001_0000, StatusFlags::BREAK.bits());
    }
    #[test]
    fn test_sec() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x38, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, StatusFlags::CARRY.bits());
    }
    #[test]
    fn test_clc() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x18, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
    }
    #[test]
    fn test_sed() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xf8, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_1000, 0b0000_1000);
    }
    #[test]
    fn test_cld() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xD8, 0x00]);
        cpu.status = StatusFlags::from_bits_truncate(0b0000_1000);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_1000, 0);
    }
    #[test]
    fn test_sei() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x78, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0100, 0b0000_0100);
    }
    #[test]
    fn test_cli() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x58, 0x00]);
        cpu.status = StatusFlags::from_bits_truncate(0b0000_0100);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0000_0100, 0);
    }
    #[test]
    fn test_clv() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xB8, 0x00]);
        cpu.status = StatusFlags::from_bits_truncate(0b0100_0000);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.status.bits() & 0b0100_0000, 0);
    }
}
