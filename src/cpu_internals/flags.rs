use bitflags::bitflags;

bitflags! {
    /// NVRB_DIZC (R 予約済み　使用できない)
    /// N: negative
    /// V: overflow
    /// R: reserved
    /// B: break command
    /// D: decimal mode
    /// I: interrupt diable
    /// Z: zero flag set if value == 0
    /// C: Carry
    #[derive(Clone, Debug, Copy)]
    pub struct StatusFlags: u8 {
        const CARRY = 1;
        const ZERO = 1 << 1;
        const INTERRUPT_DISABLE = 1 << 2;
        const DECIMAL_MODE = 1 << 3;
        const BREAK = 1 << 4;
        const RESERVED = 1 << 5;
        const OVERFLOW = 1 << 6;
        const NEGATIVE = 1 << 7;
    }
}
