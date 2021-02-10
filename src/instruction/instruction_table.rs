use super::util::{
    Mnemonic,
    AddressingMode
};
use super::arithmetic::{
    add,
    sub,
    inc,
    inx,
    iny,
    dec,
    dex,
    dey,
};
use super::bitwise::{
    and,
    ora,
    eor,
    asl,
};


pub const MNEMONICS: [Mnemonic;57] = [
    // MNEMONIC:ADC
    Mnemonic { name:"ADC", opcode: 0x61, addr_mode: AddressingMode::IndexedIndirect, func: add},
    Mnemonic { name:"ADC", opcode: 0x65, addr_mode: AddressingMode::ZeroPage, func: add },
    Mnemonic { name:"ADC", opcode: 0x69, addr_mode: AddressingMode::Immediate, func: add},
    Mnemonic { name:"ADC", opcode: 0x6D, addr_mode: AddressingMode::Absolute, func: add},
    Mnemonic { name:"ADC", opcode: 0x71, addr_mode: AddressingMode::IndirectIndexed, func: add},
    Mnemonic { name:"ADC", opcode: 0x75, addr_mode: AddressingMode::ZeroPageIndexedX, func: add},
    Mnemonic { name:"ADC", opcode: 0x79, addr_mode: AddressingMode::AbsoluteIndexedY, func: add},
    Mnemonic { name:"ADC", opcode: 0x7D, addr_mode: AddressingMode::AbsoluteIndexedX, func: add},
    // MNEMONIC:SBC
    Mnemonic { name:"SBC", opcode: 0xE1, addr_mode: AddressingMode::IndexedIndirect, func: sub},
    Mnemonic { name:"SBC", opcode: 0xE5, addr_mode: AddressingMode::ZeroPage, func: sub},
    Mnemonic { name:"SBC", opcode: 0xE9, addr_mode: AddressingMode::Immediate, func: sub},
    Mnemonic { name:"SBC", opcode: 0xED, addr_mode: AddressingMode::Absolute, func: sub},
    Mnemonic { name:"SBC", opcode: 0xF1, addr_mode: AddressingMode::IndirectIndexed, func: sub},
    Mnemonic { name:"SBC", opcode: 0xF5, addr_mode: AddressingMode::ZeroPageIndexedX, func: sub},
    Mnemonic { name:"SBC", opcode: 0xF9, addr_mode: AddressingMode::AbsoluteIndexedY, func: sub},
    Mnemonic { name:"SBC", opcode: 0xFD, addr_mode: AddressingMode::AbsoluteIndexedX, func: sub},
    // MNEMONIC:INC
    Mnemonic { name:"INC", opcode: 0xE6, addr_mode: AddressingMode::ZeroPage, func: inc},
    Mnemonic { name:"INC", opcode: 0xEE, addr_mode: AddressingMode::Absolute, func: inc},
    Mnemonic { name:"INC", opcode: 0xF6, addr_mode: AddressingMode::ZeroPageIndexedX, func: inc},
    Mnemonic { name:"INC", opcode: 0xFE, addr_mode: AddressingMode::AbsoluteIndexedX, func: inc},
    // MNEMONIC:INX
    Mnemonic { name:"INX", opcode: 0xE8, addr_mode: AddressingMode::Implied, func: inx},
    // MNEMONIC:INY
    Mnemonic { name:"INY", opcode: 0xC8, addr_mode: AddressingMode::Implied, func: iny},
    // MNEMONIC:DEC
    Mnemonic { name:"DEC", opcode: 0xC6, addr_mode: AddressingMode::ZeroPage, func: dec},
    Mnemonic { name:"DEC", opcode: 0xCE, addr_mode: AddressingMode::Absolute, func: dec},
    Mnemonic { name:"DEC", opcode: 0xD6, addr_mode: AddressingMode::ZeroPageIndexedX, func: dec},
    Mnemonic { name:"DEC", opcode: 0xDE, addr_mode: AddressingMode::AbsoluteIndexedX, func: dec},
    // MNEMONIC:DEX
    Mnemonic { name:"DEX", opcode: 0xCA, addr_mode: AddressingMode::Implied, func: dex},
    // MNEMONIC:DEY
    Mnemonic { name:"DEY", opcode: 0x88, addr_mode: AddressingMode::Implied, func: dey},
    // MNEMONIC:AND
    Mnemonic { name:"AND", opcode: 0x21, addr_mode: AddressingMode::IndexedIndirect, func: and},
    Mnemonic { name:"AND", opcode: 0x25, addr_mode: AddressingMode::ZeroPage, func: and},
    Mnemonic { name:"AND", opcode: 0x29, addr_mode: AddressingMode::Immediate, func: and},
    Mnemonic { name:"AND", opcode: 0x2D, addr_mode: AddressingMode::Absolute, func: and},
    Mnemonic { name:"AND", opcode: 0x31, addr_mode: AddressingMode::IndirectIndexed, func: and},
    Mnemonic { name:"AND", opcode: 0x35, addr_mode: AddressingMode::ZeroPageIndexedX, func: and},
    Mnemonic { name:"AND", opcode: 0x39, addr_mode: AddressingMode::AbsoluteIndexedY, func: and},
    Mnemonic { name:"AND", opcode: 0x3D, addr_mode: AddressingMode::AbsoluteIndexedX, func: and},
    // MNEMONIC:ORA
    Mnemonic { name:"ORA", opcode: 0x01, addr_mode: AddressingMode::IndexedIndirect, func: ora},
    Mnemonic { name:"ORA", opcode: 0x05, addr_mode: AddressingMode::ZeroPage, func: ora},
    Mnemonic { name:"ORA", opcode: 0x09, addr_mode: AddressingMode::Immediate, func: ora},
    Mnemonic { name:"ORA", opcode: 0x0D, addr_mode: AddressingMode::Absolute, func: ora},
    Mnemonic { name:"ORA", opcode: 0x11, addr_mode: AddressingMode::IndirectIndexed, func: ora},
    Mnemonic { name:"ORA", opcode: 0x15, addr_mode: AddressingMode::ZeroPageIndexedX, func: ora},
    Mnemonic { name:"ORA", opcode: 0x19, addr_mode: AddressingMode::AbsoluteIndexedY, func: ora},
    Mnemonic { name:"ORA", opcode: 0x1D, addr_mode: AddressingMode::AbsoluteIndexedX, func: ora},
    // MNEMONIC:EOR
    Mnemonic { name:"EOR", opcode: 0x01, addr_mode: AddressingMode::IndexedIndirect, func: eor},
    Mnemonic { name:"EOR", opcode: 0x05, addr_mode: AddressingMode::ZeroPage, func: eor},
    Mnemonic { name:"EOR", opcode: 0x09, addr_mode: AddressingMode::Immediate, func: eor},
    Mnemonic { name:"EOR", opcode: 0x0D, addr_mode: AddressingMode::Absolute, func: eor},
    Mnemonic { name:"EOR", opcode: 0x11, addr_mode: AddressingMode::IndirectIndexed, func: eor},
    Mnemonic { name:"EOR", opcode: 0x15, addr_mode: AddressingMode::ZeroPageIndexedX, func: eor},
    Mnemonic { name:"EOR", opcode: 0x19, addr_mode: AddressingMode::AbsoluteIndexedY, func: eor},
    Mnemonic { name:"EOR", opcode: 0x1D, addr_mode: AddressingMode::AbsoluteIndexedX, func: eor},
    // MNEMONIC:ASL
    Mnemonic { name:"ASL", opcode: 0x06, addr_mode: AddressingMode::ZeroPage, func: asl},
    Mnemonic { name:"ASL", opcode: 0x0A, addr_mode: AddressingMode::Accumulator, func: asl},
    Mnemonic { name:"ASL", opcode: 0x0E, addr_mode: AddressingMode::Absolute, func: asl},
    Mnemonic { name:"ASL", opcode: 0x16, addr_mode: AddressingMode::ZeroPageIndexedX, func: asl},
    Mnemonic { name:"ASL", opcode: 0x1E, addr_mode: AddressingMode::AbsoluteIndexedX, func: asl},
    ];