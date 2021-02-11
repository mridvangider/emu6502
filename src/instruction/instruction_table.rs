use super::util::{
    Mnemonic,
    AddressingMode
};
use super::arithmetic::*;
use super::bitwise::*;
use super::comparison::*;
use super::laodstore::*;
use super::branch::*;
use super::transfer::*;

pub const MNEMONICS: [Mnemonic;133] = [
    // MNEMONIC:ADC
    Mnemonic { name:"ADC", opcode: 0x61, addr_mode: AddressingMode::IndexedIndirect, func: add },
    Mnemonic { name:"ADC", opcode: 0x65, addr_mode: AddressingMode::ZeroPage, func: add },
    Mnemonic { name:"ADC", opcode: 0x69, addr_mode: AddressingMode::Immediate, func: add },
    Mnemonic { name:"ADC", opcode: 0x6D, addr_mode: AddressingMode::Absolute, func: add },
    Mnemonic { name:"ADC", opcode: 0x71, addr_mode: AddressingMode::IndirectIndexed, func: add },
    Mnemonic { name:"ADC", opcode: 0x75, addr_mode: AddressingMode::ZeroPageIndexedX, func: add },
    Mnemonic { name:"ADC", opcode: 0x79, addr_mode: AddressingMode::AbsoluteIndexedY, func: add },
    Mnemonic { name:"ADC", opcode: 0x7D, addr_mode: AddressingMode::AbsoluteIndexedX, func: add },
    // MNEMONIC:SBC
    Mnemonic { name:"SBC", opcode: 0xE1, addr_mode: AddressingMode::IndexedIndirect, func: sub },
    Mnemonic { name:"SBC", opcode: 0xE5, addr_mode: AddressingMode::ZeroPage, func: sub },
    Mnemonic { name:"SBC", opcode: 0xE9, addr_mode: AddressingMode::Immediate, func: sub },
    Mnemonic { name:"SBC", opcode: 0xED, addr_mode: AddressingMode::Absolute, func: sub },
    Mnemonic { name:"SBC", opcode: 0xF1, addr_mode: AddressingMode::IndirectIndexed, func: sub },
    Mnemonic { name:"SBC", opcode: 0xF5, addr_mode: AddressingMode::ZeroPageIndexedX, func: sub },
    Mnemonic { name:"SBC", opcode: 0xF9, addr_mode: AddressingMode::AbsoluteIndexedY, func: sub },
    Mnemonic { name:"SBC", opcode: 0xFD, addr_mode: AddressingMode::AbsoluteIndexedX, func: sub },
    // MNEMONIC:INC
    Mnemonic { name:"INC", opcode: 0xE6, addr_mode: AddressingMode::ZeroPage, func: inc },
    Mnemonic { name:"INC", opcode: 0xEE, addr_mode: AddressingMode::Absolute, func: inc },
    Mnemonic { name:"INC", opcode: 0xF6, addr_mode: AddressingMode::ZeroPageIndexedX, func: inc },
    Mnemonic { name:"INC", opcode: 0xFE, addr_mode: AddressingMode::AbsoluteIndexedX, func: inc },
    // MNEMONIC:INX
    Mnemonic { name:"INX", opcode: 0xE8, addr_mode: AddressingMode::Implied, func: inx },
    // MNEMONIC:INY
    Mnemonic { name:"INY", opcode: 0xC8, addr_mode: AddressingMode::Implied, func: iny },
    // MNEMONIC:DEC
    Mnemonic { name:"DEC", opcode: 0xC6, addr_mode: AddressingMode::ZeroPage, func: dec },
    Mnemonic { name:"DEC", opcode: 0xCE, addr_mode: AddressingMode::Absolute, func: dec },
    Mnemonic { name:"DEC", opcode: 0xD6, addr_mode: AddressingMode::ZeroPageIndexedX, func: dec },
    Mnemonic { name:"DEC", opcode: 0xDE, addr_mode: AddressingMode::AbsoluteIndexedX, func: dec },
    // MNEMONIC:DEX
    Mnemonic { name:"DEX", opcode: 0xCA, addr_mode: AddressingMode::Implied, func: dex },
    // MNEMONIC:DEY
    Mnemonic { name:"DEY", opcode: 0x88, addr_mode: AddressingMode::Implied, func: dey },
    // MNEMONIC:AND
    Mnemonic { name:"AND", opcode: 0x21, addr_mode: AddressingMode::IndexedIndirect, func: and },
    Mnemonic { name:"AND", opcode: 0x25, addr_mode: AddressingMode::ZeroPage, func: and },
    Mnemonic { name:"AND", opcode: 0x29, addr_mode: AddressingMode::Immediate, func: and },
    Mnemonic { name:"AND", opcode: 0x2D, addr_mode: AddressingMode::Absolute, func: and },
    Mnemonic { name:"AND", opcode: 0x31, addr_mode: AddressingMode::IndirectIndexed, func: and },
    Mnemonic { name:"AND", opcode: 0x35, addr_mode: AddressingMode::ZeroPageIndexedX, func: and },
    Mnemonic { name:"AND", opcode: 0x39, addr_mode: AddressingMode::AbsoluteIndexedY, func: and },
    Mnemonic { name:"AND", opcode: 0x3D, addr_mode: AddressingMode::AbsoluteIndexedX, func: and },
    // MNEMONIC:ORA
    Mnemonic { name:"ORA", opcode: 0x01, addr_mode: AddressingMode::IndexedIndirect, func: ora },
    Mnemonic { name:"ORA", opcode: 0x05, addr_mode: AddressingMode::ZeroPage, func: ora },
    Mnemonic { name:"ORA", opcode: 0x09, addr_mode: AddressingMode::Immediate, func: ora },
    Mnemonic { name:"ORA", opcode: 0x0D, addr_mode: AddressingMode::Absolute, func: ora },
    Mnemonic { name:"ORA", opcode: 0x11, addr_mode: AddressingMode::IndirectIndexed, func: ora },
    Mnemonic { name:"ORA", opcode: 0x15, addr_mode: AddressingMode::ZeroPageIndexedX, func: ora },
    Mnemonic { name:"ORA", opcode: 0x19, addr_mode: AddressingMode::AbsoluteIndexedY, func: ora },
    Mnemonic { name:"ORA", opcode: 0x1D, addr_mode: AddressingMode::AbsoluteIndexedX, func: ora },
    // MNEMONIC:EOR
    Mnemonic { name:"EOR", opcode: 0x01, addr_mode: AddressingMode::IndexedIndirect, func: eor },
    Mnemonic { name:"EOR", opcode: 0x05, addr_mode: AddressingMode::ZeroPage, func: eor },
    Mnemonic { name:"EOR", opcode: 0x09, addr_mode: AddressingMode::Immediate, func: eor },
    Mnemonic { name:"EOR", opcode: 0x0D, addr_mode: AddressingMode::Absolute, func: eor },
    Mnemonic { name:"EOR", opcode: 0x11, addr_mode: AddressingMode::IndirectIndexed, func: eor },
    Mnemonic { name:"EOR", opcode: 0x15, addr_mode: AddressingMode::ZeroPageIndexedX, func: eor },
    Mnemonic { name:"EOR", opcode: 0x19, addr_mode: AddressingMode::AbsoluteIndexedY, func: eor },
    Mnemonic { name:"EOR", opcode: 0x1D, addr_mode: AddressingMode::AbsoluteIndexedX, func: eor },
    // MNEMONIC:ASL
    Mnemonic { name:"ASL", opcode: 0x06, addr_mode: AddressingMode::ZeroPage, func: asl },
    Mnemonic { name:"ASL", opcode: 0x0A, addr_mode: AddressingMode::Accumulator, func: asl },
    Mnemonic { name:"ASL", opcode: 0x0E, addr_mode: AddressingMode::Absolute, func: asl },
    Mnemonic { name:"ASL", opcode: 0x16, addr_mode: AddressingMode::ZeroPageIndexedX, func: asl },
    Mnemonic { name:"ASL", opcode: 0x1E, addr_mode: AddressingMode::AbsoluteIndexedX, func: asl },
    // MNEMONIC:LSR
    Mnemonic { name:"LSR", opcode: 0x46, addr_mode: AddressingMode::ZeroPage, func: lsr },
    Mnemonic { name:"LSR", opcode: 0x4A, addr_mode: AddressingMode::Accumulator, func: lsr },
    Mnemonic { name:"LSR", opcode: 0x4E, addr_mode: AddressingMode::Absolute, func: lsr },
    Mnemonic { name:"LSR", opcode: 0x56, addr_mode: AddressingMode::ZeroPageIndexedX, func: lsr },
    Mnemonic { name:"LSR", opcode: 0x5E, addr_mode: AddressingMode::AbsoluteIndexedX, func: lsr },
    // MNEMONIC:ROL
    Mnemonic { name:"ROL", opcode: 0x26, addr_mode: AddressingMode::ZeroPage, func: rol },
    Mnemonic { name:"ROL", opcode: 0x2A, addr_mode: AddressingMode::Accumulator, func: rol },
    Mnemonic { name:"ROL", opcode: 0x2E, addr_mode: AddressingMode::Absolute, func: rol },
    Mnemonic { name:"ROL", opcode: 0x36, addr_mode: AddressingMode::ZeroPageIndexedX, func: rol },
    Mnemonic { name:"ROL", opcode: 0x3E, addr_mode: AddressingMode::AbsoluteIndexedX, func: rol },
    // MNEMONIC:ROR
    Mnemonic { name:"ROR", opcode: 0x66, addr_mode: AddressingMode::ZeroPage, func: ror },
    Mnemonic { name:"ROR", opcode: 0x6A, addr_mode: AddressingMode::Accumulator, func: ror },
    Mnemonic { name:"ROR", opcode: 0x6E, addr_mode: AddressingMode::Absolute, func: ror },
    Mnemonic { name:"ROR", opcode: 0x76, addr_mode: AddressingMode::ZeroPageIndexedX, func: ror },
    Mnemonic { name:"ROR", opcode: 0x7E, addr_mode: AddressingMode::AbsoluteIndexedX, func: ror },
    // MNEMONIC:CMP
    Mnemonic { name:"CMP", opcode: 0xC1, addr_mode: AddressingMode::IndexedIndirect, func: cmp },
    Mnemonic { name:"CMP", opcode: 0xC5, addr_mode: AddressingMode::ZeroPage, func: cmp },
    Mnemonic { name:"CMP", opcode: 0xC9, addr_mode: AddressingMode::Immediate, func: cmp },
    Mnemonic { name:"CMP", opcode: 0xCD, addr_mode: AddressingMode::Absolute, func: cmp },
    Mnemonic { name:"CMP", opcode: 0xD1, addr_mode: AddressingMode::IndirectIndexed, func: cmp },
    Mnemonic { name:"CMP", opcode: 0xD5, addr_mode: AddressingMode::ZeroPageIndexedX, func: cmp },
    Mnemonic { name:"CMP", opcode: 0xD9, addr_mode: AddressingMode::AbsoluteIndexedY, func: cmp },
    Mnemonic { name:"CMP", opcode: 0xDD, addr_mode: AddressingMode::AbsoluteIndexedX, func: cmp },
    // MNEMONIC:CPX
    Mnemonic { name:"CPX", opcode: 0xE0, addr_mode: AddressingMode::Immediate, func: cpx },
    Mnemonic { name:"CPX", opcode: 0xE4, addr_mode: AddressingMode::ZeroPage, func: cpx },
    Mnemonic { name:"CPX", opcode: 0xEC, addr_mode: AddressingMode::Absolute, func: cpx },
    // MNEMONIC:CPY
    Mnemonic { name:"CPY", opcode: 0xC0, addr_mode: AddressingMode::Immediate, func: cpy },
    Mnemonic { name:"CPY", opcode: 0xC4, addr_mode: AddressingMode::ZeroPage, func: cpy },
    Mnemonic { name:"CPY", opcode: 0xCC, addr_mode: AddressingMode::Absolute, func: cpy },
    // MNEMONIC:BIT
    Mnemonic { name:"BIT", opcode: 0x24, addr_mode: AddressingMode::ZeroPage, func: bit },
    Mnemonic { name:"BIT", opcode: 0x2C, addr_mode: AddressingMode::Absolute, func: bit },
    // MNEMONIC:LDA
    Mnemonic { name:"LDA", opcode: 0xA1, addr_mode: AddressingMode::IndexedIndirect, func: lda },
    Mnemonic { name:"LDA", opcode: 0xA5, addr_mode: AddressingMode::ZeroPage, func: lda },
    Mnemonic { name:"LDA", opcode: 0xA9, addr_mode: AddressingMode::Immediate, func: lda },
    Mnemonic { name:"LDA", opcode: 0xAD, addr_mode: AddressingMode::Absolute, func: lda },
    Mnemonic { name:"LDA", opcode: 0xB1, addr_mode: AddressingMode::IndirectIndexed, func: lda },
    Mnemonic { name:"LDA", opcode: 0xB5, addr_mode: AddressingMode::ZeroPageIndexedX, func: lda },
    Mnemonic { name:"LDA", opcode: 0xB9, addr_mode: AddressingMode::AbsoluteIndexedY, func: lda },
    Mnemonic { name:"LDA", opcode: 0xBD, addr_mode: AddressingMode::AbsoluteIndexedX, func: lda },
    // MNEMONIC:LDX
    Mnemonic { name:"LDX", opcode: 0xA2, addr_mode: AddressingMode::Immediate, func: ldx },
    Mnemonic { name:"LDX", opcode: 0xA6, addr_mode: AddressingMode::ZeroPage, func: ldx },
    Mnemonic { name:"LDX", opcode: 0xAE, addr_mode: AddressingMode::Absolute, func: ldx },
    Mnemonic { name:"LDX", opcode: 0xB6, addr_mode: AddressingMode::ZeroPageIndexedY, func: ldx },
    Mnemonic { name:"LDX", opcode: 0xBE, addr_mode: AddressingMode::AbsoluteIndexedY, func: ldx },
    // MNEMONIC:LDY
    Mnemonic { name:"LDY", opcode: 0xA0, addr_mode: AddressingMode::Immediate, func: ldy },
    Mnemonic { name:"LDY", opcode: 0xA4, addr_mode: AddressingMode::ZeroPage, func: ldy },
    Mnemonic { name:"LDY", opcode: 0xAC, addr_mode: AddressingMode::Absolute, func: ldy },
    Mnemonic { name:"LDY", opcode: 0xB4, addr_mode: AddressingMode::ZeroPageIndexedX, func: ldy },
    Mnemonic { name:"LDY", opcode: 0xBC, addr_mode: AddressingMode::AbsoluteIndexedX, func: ldy },
    // MNEMONIC:STA
    Mnemonic { name:"STA", opcode: 0x81, addr_mode: AddressingMode::IndexedIndirect, func: sta },
    Mnemonic { name:"STA", opcode: 0x85, addr_mode: AddressingMode::ZeroPage, func: sta },
    Mnemonic { name:"STA", opcode: 0x8D, addr_mode: AddressingMode::Absolute, func: sta },
    Mnemonic { name:"STA", opcode: 0x91, addr_mode: AddressingMode::IndirectIndexed, func: sta },
    Mnemonic { name:"STA", opcode: 0x95, addr_mode: AddressingMode::ZeroPageIndexedX, func: sta },
    Mnemonic { name:"STA", opcode: 0x99, addr_mode: AddressingMode::AbsoluteIndexedY, func: sta },
    Mnemonic { name:"STA", opcode: 0x9D, addr_mode: AddressingMode::AbsoluteIndexedX, func: sta },
    // MNEMONIC:STX
    Mnemonic { name:"STX", opcode: 0x86, addr_mode: AddressingMode::ZeroPage, func: stx },
    Mnemonic { name:"STX", opcode: 0x8E, addr_mode: AddressingMode::Absolute, func: stx },
    Mnemonic { name:"STX", opcode: 0x96, addr_mode: AddressingMode::ZeroPageIndexedY, func: stx },
    // MNEMONIC:STY
    Mnemonic { name:"STY", opcode: 0x84, addr_mode: AddressingMode::ZeroPage, func: sty },
    Mnemonic { name:"STY", opcode: 0x8C, addr_mode: AddressingMode::Absolute, func: sty },
    Mnemonic { name:"STY", opcode: 0x94, addr_mode: AddressingMode::ZeroPageIndexedX, func: sty },
    // MNEMONIC:BCC
    Mnemonic { name:"BCC", opcode: 0x90, addr_mode: AddressingMode::Relative, func: bcc },
    // MNEMONIC:BCS
    Mnemonic { name:"BCS", opcode: 0xB0, addr_mode: AddressingMode::Relative, func: bcs },
    // MNEMONIC:BNE
    Mnemonic { name:"BNE", opcode: 0xD0, addr_mode: AddressingMode::Relative, func: bne },
    // MNEMONIC:BEQ
    Mnemonic { name:"BEQ", opcode: 0xF0, addr_mode: AddressingMode::Relative, func: beq },
    // MNEMONIC:BPL
    Mnemonic { name:"BPL", opcode: 0x10, addr_mode: AddressingMode::Relative, func: bpl },
    // MNEMONIC:BMI
    Mnemonic { name:"BMI", opcode: 0x30, addr_mode: AddressingMode::Relative, func: bmi },
    // MNEMONIC:BVC
    Mnemonic { name:"BVC", opcode: 0x90, addr_mode: AddressingMode::Relative, func: bvc },
    // MNEMONIC:BVS
    Mnemonic { name:"BVS", opcode: 0x90, addr_mode: AddressingMode::Relative, func: bvs },
    // MNEMONIC:TAX
    Mnemonic { name:"TAX", opcode: 0xAA, addr_mode: AddressingMode::Implied, func: tax },
    // MNEMONIC:TXA
    Mnemonic { name:"TXA", opcode: 0x8A, addr_mode: AddressingMode::Implied, func: txa },
    // MNEMONIC:TAY
    Mnemonic { name:"TAY", opcode: 0xA8, addr_mode: AddressingMode::Implied, func: tay },
    // MNEMONIC:TYA
    Mnemonic { name:"TYA", opcode: 0x98, addr_mode: AddressingMode::Implied, func: tya },
    // MNEMONIC:TSX
    Mnemonic { name:"TSX", opcode: 0xBA, addr_mode: AddressingMode::Implied, func: tsx },
    // MNEMONIC:TXS
    Mnemonic { name:"TXS", opcode: 0x9A, addr_mode: AddressingMode::Implied, func: txs },
    ];