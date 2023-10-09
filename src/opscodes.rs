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
        OpCode::new("TAX", 0xaa, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("INX", 0xe8, 1, 2, AddressingMode::NonAddressing),
        // LDA
        OpCode::new("LDA", 0xa9, 2, 2, AddressingMode::Immediate),
        OpCode::new("LDA", 0xa5, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("LDA", 0xb5, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("LDA", 0xad, 3, 4, AddressingMode::Absolute),
        OpCode::new("LDA", 0xbd, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("LDA", 0xb9, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("LDA", 0xa1, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("LDA", 0xb1, 2, 5, AddressingMode::Indirect_Y),
        // STA
        OpCode::new("STA", 0x85, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("STA", 0x95, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("STA", 0x8D, 3, 4, AddressingMode::Absolute),
        OpCode::new("STA", 0x9D, 3, 5, AddressingMode::Absolute_X),
        OpCode::new("STA", 0x99, 3, 5, AddressingMode::Absolute_Y),
        OpCode::new("STA", 0x81, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("STA", 0x91, 2, 6, AddressingMode::Indirect_Y),
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
    ]
});
pub static OPCODES_MAP: Lazy<HashMap<u8, &'static OpCode>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for op in &*CPU_OPS_CODES {
        map.insert(op.code, op);
    }
    map
});
