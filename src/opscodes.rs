use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NonAddressing,
}
pub trait AddressingModeConverter {
    fn get_operand_address(&self, mode: &AddressingMode) -> u16;
}

pub struct OpCode {
    pub name: &'static str,
    pub code: u8,
    pub bytes_len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(
        name: &'static str,
        code: u8,
        bytes_len: u8,
        cycles: u8,
        mode: AddressingMode,
    ) -> OpCode {
        OpCode {
            name,
            code,
            bytes_len,
            cycles,
            mode,
        }
    }
}

pub static CPU_OPS_CODES: Lazy<Vec<OpCode>> = Lazy::new(|| {
    vec![
        OpCode::new("BRK", 0x00, 1, 7, AddressingMode::NonAddressing),
        // BCS
        OpCode::new("BCS", 0xb0, 2, 2, AddressingMode::NonAddressing),
        // BCC
        OpCode::new("BCC", 0x90, 2, 2, AddressingMode::NonAddressing),
        // BEQ
        OpCode::new("BEQ", 0xf0, 2, 2, AddressingMode::NonAddressing),
        // BNE
        OpCode::new("BNE", 0xd0, 2, 2, AddressingMode::NonAddressing),
        // TSX
        OpCode::new("TSX", 0xba, 1, 2, AddressingMode::NonAddressing),
        // TXS
        OpCode::new("TXS", 0x9a, 1, 2, AddressingMode::NonAddressing),
        // INC
        OpCode::new("INC", 0xe6, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("INC", 0xf6, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("INC", 0xee, 3, 6, AddressingMode::Absolute),
        OpCode::new("INC", 0xfe, 3, 7, AddressingMode::Absolute_X),
        // INX
        OpCode::new("INX", 0xe8, 1, 2, AddressingMode::NonAddressing),
        // INY
        OpCode::new("INY", 0xc8, 1, 2, AddressingMode::NonAddressing),
        // DEC
        OpCode::new("DEC", 0xc6, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("DEC", 0xd6, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("DEC", 0xce, 3, 6, AddressingMode::Absolute),
        OpCode::new("DEC", 0xde, 3, 7, AddressingMode::Absolute_X),
        // DEX
        OpCode::new("DEX", 0xca, 1, 2, AddressingMode::NonAddressing),
        // DEY
        OpCode::new("DEY", 0x88, 1, 2, AddressingMode::NonAddressing),
        // CMP
        OpCode::new("CMP", 0xc9, 2, 2, AddressingMode::Immediate),
        OpCode::new("CMP", 0xc5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("CMP", 0xd5, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("CMP", 0xcd, 3, 4, AddressingMode::Absolute),
        OpCode::new("CMP", 0xdd, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("CMP", 0xd9, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("CMP", 0xc1, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("CMP", 0xd1, 2, 5, AddressingMode::Indirect_Y),
        // CPX
        OpCode::new("CPX", 0xe0, 2, 2, AddressingMode::Immediate),
        OpCode::new("CPX", 0xe4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("CPX", 0xec, 3, 4, AddressingMode::Absolute),
        // Cpy
        OpCode::new("CPY", 0xc0, 2, 2, AddressingMode::Immediate),
        OpCode::new("CPY", 0xc4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("CPY", 0xcc, 3, 4, AddressingMode::Absolute),
        // TAX
        OpCode::new("TAX", 0xaa, 1, 2, AddressingMode::NonAddressing),
        // TXA
        OpCode::new("TXA", 0x8a, 1, 2, AddressingMode::NonAddressing),
        // TAY
        OpCode::new("TAY", 0xa8, 1, 2, AddressingMode::NonAddressing),
        // TYA
        OpCode::new("TYA", 0x98, 1, 2, AddressingMode::NonAddressing),
        // LDA
        OpCode::new("LDA", 0xa9, 2, 2, AddressingMode::Immediate),
        OpCode::new("LDA", 0xa5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("LDA", 0xb5, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("LDA", 0xad, 3, 4, AddressingMode::Absolute),
        OpCode::new("LDA", 0xbd, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("LDA", 0xb9, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("LDA", 0xa1, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("LDA", 0xb1, 2, 5, AddressingMode::Indirect_Y),
        // LDX
        OpCode::new("LDX", 0xa2, 2, 2, AddressingMode::Immediate),
        OpCode::new("LDX", 0xa6, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("LDX", 0xb6, 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new("LDX", 0xae, 3, 4, AddressingMode::Absolute),
        OpCode::new("LDX", 0xbe, 3, 4, AddressingMode::Absolute_Y),
        // LDY
        OpCode::new("LDY", 0xa0, 2, 2, AddressingMode::Immediate),
        OpCode::new("LDY", 0xa4, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("LDY", 0xb4, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("LDY", 0xac, 3, 4, AddressingMode::Absolute),
        OpCode::new("LDY", 0xbc, 3, 4, AddressingMode::Absolute_X),
        // STA
        OpCode::new("STA", 0x85, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("STA", 0x95, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("STA", 0x8D, 3, 4, AddressingMode::Absolute),
        OpCode::new("STA", 0x9D, 3, 5, AddressingMode::Absolute_X),
        OpCode::new("STA", 0x99, 3, 5, AddressingMode::Absolute_Y),
        OpCode::new("STA", 0x81, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("STA", 0x91, 2, 6, AddressingMode::Indirect_Y),
        // STX
        OpCode::new("STX", 0x86, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("STX", 0x96, 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new("STX", 0x8e, 3, 4, AddressingMode::Absolute),
        // STY
        OpCode::new("STY", 0x84, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("STY", 0x94, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("STY", 0x8c, 3, 4, AddressingMode::Absolute),
        // ADC
        OpCode::new("ADC", 0x69, 2, 2, AddressingMode::Immediate),
        OpCode::new("ADC", 0x65, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("ADC", 0x75, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("ADC", 0x6D, 3, 4, AddressingMode::Absolute),
        OpCode::new("ADC", 0x7D, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("ADC", 0x79, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("ADC", 0x61, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("ADC", 0x71, 2, 5, AddressingMode::Indirect_Y),
        // SBC
        OpCode::new("SBC", 0xe9, 2, 2, AddressingMode::Immediate),
        OpCode::new("SBC", 0xe5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("SBC", 0xf5, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("SBC", 0xed, 3, 4, AddressingMode::Absolute),
        OpCode::new("SBC", 0xfd, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("SBC", 0xf9, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("SBC", 0xe1, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("SBC", 0xf1, 2, 5, AddressingMode::Indirect_Y),
        // AND
        OpCode::new("AND", 0x29, 2, 2, AddressingMode::Immediate),
        OpCode::new("AND", 0x25, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("AND", 0x35, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("AND", 0x2d, 3, 4, AddressingMode::Absolute),
        OpCode::new("AND", 0x3d, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("AND", 0x39, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("AND", 0x21, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("AND", 0x31, 2, 5, AddressingMode::Indirect_Y),
        // EOR
        OpCode::new("EOR", 0x49, 2, 2, AddressingMode::Immediate),
        OpCode::new("EOR", 0x45, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("EOR", 0x55, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("EOR", 0x4d, 3, 4, AddressingMode::Absolute),
        OpCode::new("EOR", 0x5d, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("EOR", 0x59, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("EOR", 0x41, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("EOR", 0x51, 2, 5, AddressingMode::Indirect_Y),
        // ORA
        OpCode::new("ORA", 0x09, 2, 2, AddressingMode::Immediate),
        OpCode::new("ORA", 0x05, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("ORA", 0x15, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("ORA", 0x0d, 3, 4, AddressingMode::Absolute),
        OpCode::new("ORA", 0x1d, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("ORA", 0x19, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("ORA", 0x01, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("ORA", 0x11, 2, 5, AddressingMode::Indirect_Y),
        // BIT
        OpCode::new("BIT", 0x24, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("BIT", 0x2c, 3, 4, AddressingMode::Absolute),
        // BRK
        OpCode::new("BRK", 0x00, 1, 7, AddressingMode::NonAddressing),
        // SEC
        OpCode::new("SEC", 0x38, 1, 2, AddressingMode::NonAddressing),
        // SED
        OpCode::new("SED", 0xf8, 1, 2, AddressingMode::NonAddressing),
        // CEC
        OpCode::new("CEC", 0x18, 1, 2, AddressingMode::NonAddressing),
        // CLD
        OpCode::new("CLD", 0xd8, 1, 2, AddressingMode::NonAddressing),
        // SEI
        OpCode::new("SEI", 0x78, 1, 2, AddressingMode::NonAddressing),
        // CLI
        OpCode::new("CLI", 0x58, 1, 2, AddressingMode::NonAddressing),
        // CLV
        OpCode::new("CLV", 0xb8, 1, 2, AddressingMode::NonAddressing),
        // NOP
        OpCode::new("NOP", 0xea, 1, 2, AddressingMode::NonAddressing),
    ]
});
pub static OPCODES_MAP: Lazy<HashMap<u8, &'static OpCode>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for op in &*CPU_OPS_CODES {
        map.insert(op.code, op);
    }
    map
});
