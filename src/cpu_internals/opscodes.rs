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
    Relative,
    NonAddressing,
}
pub trait AddressingModeConverter {
    fn get_absolute_address(&mut self, mode: &AddressingMode, addr: u16) -> (u16, bool);
    fn get_operand_address(&mut self, mode: &AddressingMode) -> (u16, bool);
}

#[derive(Debug)]
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
        OpCode::new("BCS", 0xb0, 2, 2, AddressingMode::Relative),
        // BCC
        OpCode::new("BCC", 0x90, 2, 2, AddressingMode::Relative),
        // BEQ
        OpCode::new("BEQ", 0xf0, 2, 2, AddressingMode::Relative),
        // BVC
        OpCode::new("BVC", 0x50, 2, 2, AddressingMode::Relative),
        // BVS
        OpCode::new("BVS", 0x70, 2, 2, AddressingMode::Relative),
        // BNE
        OpCode::new("BNE", 0xd0, 2, 2, AddressingMode::Relative),
        // BPL
        OpCode::new("BPL", 0x10, 2, 2, AddressingMode::Relative),
        // BMI
        OpCode::new("BMI", 0x30, 2, 2, AddressingMode::Relative),
        // TSX
        OpCode::new("TSX", 0xba, 1, 2, AddressingMode::NonAddressing),
        // TXS
        OpCode::new("TXS", 0x9a, 1, 2, AddressingMode::NonAddressing),
        // JSR
        OpCode::new("JSR", 0x20, 3, 6, AddressingMode::NonAddressing),
        // RTS
        OpCode::new("RTS", 0x60, 1, 6, AddressingMode::NonAddressing),
        // RTI
        OpCode::new("RTI", 0x40, 1, 6, AddressingMode::NonAddressing),
        // PHA
        OpCode::new("PHA", 0x48, 1, 3, AddressingMode::NonAddressing),
        // PLA
        OpCode::new("PLA", 0x68, 1, 4, AddressingMode::NonAddressing),
        // PHP
        OpCode::new("PHP", 0x08, 1, 3, AddressingMode::NonAddressing),
        // PLP
        OpCode::new("PLP", 0x28, 1, 4, AddressingMode::NonAddressing),
        // ASL
        OpCode::new("ASL", 0x0a, 1, 2, AddressingMode::NonAddressing),
        // ASL
        OpCode::new("ASL", 0x06, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("ASL", 0x16, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("ASL", 0x0e, 3, 6, AddressingMode::Absolute),
        OpCode::new("ASL", 0x1e, 3, 7, AddressingMode::Absolute_X),
        // LSR
        OpCode::new("LSR", 0x4a, 1, 2, AddressingMode::NonAddressing),
        // LSR
        OpCode::new("LSR", 0x46, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("LSR", 0x56, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("LSR", 0x4e, 3, 6, AddressingMode::Absolute),
        OpCode::new("LSR", 0x5e, 3, 7, AddressingMode::Absolute_X),
        // JMP
        OpCode::new("JMP", 0x4c, 3, 3, AddressingMode::NonAddressing),
        OpCode::new("JMP", 0x6c, 3, 5, AddressingMode::NonAddressing),
        // ROR
        OpCode::new("ROR", 0x6a, 1, 2, AddressingMode::NonAddressing),
        // ROR
        OpCode::new("ROR", 0x66, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("ROR", 0x76, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("ROR", 0x6e, 3, 6, AddressingMode::Absolute),
        OpCode::new("ROR", 0x7e, 3, 7, AddressingMode::Absolute_X),
        // ROL
        OpCode::new("ROL", 0x2a, 1, 2, AddressingMode::NonAddressing),
        // ROL
        OpCode::new("ROL", 0x26, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("ROL", 0x36, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("ROL", 0x2e, 3, 6, AddressingMode::Absolute),
        OpCode::new("ROL", 0x3e, 3, 7, AddressingMode::Absolute_X),
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
        OpCode::new("CLC", 0x18, 1, 2, AddressingMode::NonAddressing),
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
        // UnOfficial
        // *DCP
        OpCode::new("*DCP", 0xc7, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("*DCP", 0xd7, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("*DCP", 0xcf, 3, 6, AddressingMode::Absolute),
        OpCode::new("*DCP", 0xdf, 3, 7, AddressingMode::Absolute_X),
        OpCode::new("*DCP", 0xdb, 3, 7, AddressingMode::Absolute_Y),
        OpCode::new("*DCP", 0xc3, 2, 8, AddressingMode::Indirect_X),
        OpCode::new("*DCP", 0xd3, 2, 8, AddressingMode::Indirect_Y),
        // *ISC
        OpCode::new("*ISC", 0xe7, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("*ISC", 0xf7, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("*ISC", 0xef, 3, 6, AddressingMode::Absolute),
        OpCode::new("*ISC", 0xff, 3, 7, AddressingMode::Absolute_X),
        OpCode::new("*ISC", 0xfb, 3, 7, AddressingMode::Absolute_Y),
        OpCode::new("*ISC", 0xe3, 2, 8, AddressingMode::Indirect_X),
        OpCode::new("*ISC", 0xf3, 2, 8, AddressingMode::Indirect_Y),
        // *ISB
        OpCode::new("*ISB", 0xe7, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("*ISB", 0xf7, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("*ISB", 0xef, 3, 6, AddressingMode::Absolute),
        OpCode::new("*ISB", 0xff, 3, 7, AddressingMode::Absolute_X),
        OpCode::new("*ISB", 0xfb, 3, 7, AddressingMode::Absolute_Y),
        OpCode::new("*ISB", 0xe3, 2, 8, AddressingMode::Indirect_X),
        OpCode::new("*ISB", 0xf3, 2, 8, AddressingMode::Indirect_Y),
        // *SLO
        OpCode::new("*SLO", 0x07, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("*SLO", 0x17, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("*SLO", 0x0f, 3, 6, AddressingMode::Absolute),
        OpCode::new("*SLO", 0x1f, 3, 7, AddressingMode::Absolute_X),
        OpCode::new("*SLO", 0x1b, 3, 7, AddressingMode::Absolute_Y),
        OpCode::new("*SLO", 0x03, 2, 8, AddressingMode::Indirect_X),
        OpCode::new("*SLO", 0x13, 2, 8, AddressingMode::Indirect_Y),
        // *RLA
        OpCode::new("*RLA", 0x27, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("*RLA", 0x37, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("*RLA", 0x2f, 3, 6, AddressingMode::Absolute),
        OpCode::new("*RLA", 0x3f, 3, 7, AddressingMode::Absolute_X),
        OpCode::new("*RLA", 0x3b, 3, 7, AddressingMode::Absolute_Y),
        OpCode::new("*RLA", 0x23, 2, 8, AddressingMode::Indirect_X),
        OpCode::new("*RLA", 0x33, 2, 8, AddressingMode::Indirect_Y),
        // *SRE
        OpCode::new("*SRE", 0x47, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("*SRE", 0x57, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("*SRE", 0x4f, 3, 6, AddressingMode::Absolute),
        OpCode::new("*SRE", 0x5f, 3, 7, AddressingMode::Absolute_X),
        OpCode::new("*SRE", 0x5b, 3, 7, AddressingMode::Absolute_Y),
        OpCode::new("*SRE", 0x43, 2, 8, AddressingMode::Indirect_X),
        OpCode::new("*SRE", 0x53, 2, 8, AddressingMode::Indirect_Y),
        // *RRA
        OpCode::new("*RRA", 0x67, 2, 5, AddressingMode::ZeroPage),
        OpCode::new("*RRA", 0x77, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new("*RRA", 0x6f, 3, 6, AddressingMode::Absolute),
        OpCode::new("*RRA", 0x7f, 3, 7, AddressingMode::Absolute_X),
        OpCode::new("*RRA", 0x7b, 3, 7, AddressingMode::Absolute_Y),
        OpCode::new("*RRA", 0x63, 2, 8, AddressingMode::Indirect_X),
        OpCode::new("*RRA", 0x73, 2, 8, AddressingMode::Indirect_Y),
        // *AXS
        OpCode::new("*AXS", 0xCB, 2, 2, AddressingMode::Immediate),
        OpCode::new("*AXS", 0x87, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("*AXS", 0x97, 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new("*AXS", 0x8f, 3, 4, AddressingMode::Absolute),
        OpCode::new("*AXS", 0x83, 2, 6, AddressingMode::Indirect_X),
        // *LAX
        OpCode::new("*LAX", 0xa7, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("*LAX", 0xb7, 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new("*LAX", 0xaf, 3, 4, AddressingMode::Absolute),
        OpCode::new("*LAX", 0xbf, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new("*LAX", 0xa3, 2, 6, AddressingMode::Indirect_X),
        OpCode::new("*LAX", 0xb3, 2, 5, AddressingMode::Indirect_Y),
        // *LXA
        OpCode::new("*LXA", 0xab, 2, 2, AddressingMode::Immediate),
        // *XAA
        OpCode::new("*XAA", 0x8b, 2, 2, AddressingMode::Immediate),
        // *ARR
        OpCode::new("*ARR", 0x6b, 2, 2, AddressingMode::Immediate),
        // *SBC
        OpCode::new("*SBC", 0xeb, 2, 2, AddressingMode::Immediate),
        // *ANC
        OpCode::new("*ANC", 0x0b, 2, 2, AddressingMode::Immediate),
        OpCode::new("*ANC", 0x2b, 2, 2, AddressingMode::Immediate),
        // *ALR
        OpCode::new("*ALR", 0x4b, 2, 2, AddressingMode::Immediate),
        // *AHX
        OpCode::new("*AHX", 0x93, 2, 8, AddressingMode::Indirect_Y),
        OpCode::new("*AHX", 0x9f, 3, 4, AddressingMode::Absolute_Y),
        // *TAS
        OpCode::new("*TAS", 0x9b, 3, 5, AddressingMode::Absolute_Y),
        // *LAS
        OpCode::new("*LAS", 0xbb, 3, 4, AddressingMode::Absolute_Y),
        // *SHX
        OpCode::new("*SHX", 0x9e, 3, 4, AddressingMode::Absolute_Y),
        // *SHY
        OpCode::new("*SHY", 0x9c, 3, 4, AddressingMode::Absolute_X),
        // *SAX
        OpCode::new("*SAX", 0x87, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("*SAX", 0x97, 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new("*SAX", 0x8f, 3, 4, AddressingMode::Absolute),
        OpCode::new("*SAX", 0x83, 2, 6, AddressingMode::Indirect_X),
        // *NOP
        OpCode::new("*NOP", 0x04, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("*NOP", 0x44, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("*NOP", 0x64, 2, 3, AddressingMode::ZeroPage),
        OpCode::new("*NOP", 0x14, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("*NOP", 0x34, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("*NOP", 0x54, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("*NOP", 0x74, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("*NOP", 0xd4, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("*NOP", 0xf4, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new("*NOP", 0x80, 2, 2, AddressingMode::Immediate),
        OpCode::new("*NOP", 0x82, 2, 2, AddressingMode::Immediate),
        OpCode::new("*NOP", 0x89, 2, 2, AddressingMode::Immediate),
        OpCode::new("*NOP", 0xc2, 2, 2, AddressingMode::Immediate),
        OpCode::new("*NOP", 0xe2, 2, 2, AddressingMode::Immediate),
        OpCode::new("*NOP", 0x0c, 3, 4, AddressingMode::Absolute),
        OpCode::new("*NOP", 0x1c, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("*NOP", 0x3c, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("*NOP", 0x5c, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("*NOP", 0x7c, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("*NOP", 0xdc, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("*NOP", 0xfc, 3, 4, AddressingMode::Absolute_X),
        OpCode::new("*NOP", 0x02, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x12, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x22, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x32, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x42, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x52, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x62, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x72, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x92, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0xb2, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0xd2, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0xf2, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x1a, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x3a, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x5a, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0x7a, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0xda, 1, 2, AddressingMode::NonAddressing),
        OpCode::new("*NOP", 0xfa, 1, 2, AddressingMode::NonAddressing),
    ]
});
pub static OPCODES_MAP: Lazy<HashMap<u8, &'static OpCode>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for op in &*CPU_OPS_CODES {
        map.insert(op.code, op);
    }
    map
});
