mod cpu;
use crate::cpu::*;

fn main() {
    CPU::new().interpret(vec![0xA9, 0x01]);
    println!("Hello, world!");
}
