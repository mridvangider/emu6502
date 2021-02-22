use super::super::util::{
    Mnemonic,
    AddressingMode,
};
use crate::cpu::*;

pub const EMPTY_MNEM : Mnemonic = Mnemonic { name:"NOP", opcode: 0x00, addr_mode: AddressingMode::Implied, ofunc: CPU::nop };

pub const MNEMONICS: [Mnemonic;144] = [
    // MNEMONIC:ADC
    Mnemonic { name:"ADC", opcode: 0x61, addr_mode: AddressingMode::IndexedIndirect, ofunc: CPU::adc },
    Mnemonic { name:"ADC", opcode: 0x65, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::adc },
    Mnemonic { name:"ADC", opcode: 0x69, addr_mode: AddressingMode::Immediate, ofunc: CPU::adc },
    Mnemonic { name:"ADC", opcode: 0x6D, addr_mode: AddressingMode::Absolute, ofunc: CPU::adc },
    Mnemonic { name:"ADC", opcode: 0x71, addr_mode: AddressingMode::IndirectIndexed, ofunc: CPU::adc },
    Mnemonic { name:"ADC", opcode: 0x75, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::adc },
    Mnemonic { name:"ADC", opcode: 0x79, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::adc },
    Mnemonic { name:"ADC", opcode: 0x7D, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::adc },
    // MNEMONIC:SBC
    Mnemonic { name:"SBC", opcode: 0xE1, addr_mode: AddressingMode::IndexedIndirect, ofunc: CPU::sbc },
    Mnemonic { name:"SBC", opcode: 0xE5, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::sbc },
    Mnemonic { name:"SBC", opcode: 0xE9, addr_mode: AddressingMode::Immediate, ofunc: CPU::sbc },
    Mnemonic { name:"SBC", opcode: 0xED, addr_mode: AddressingMode::Absolute, ofunc: CPU::sbc },
    Mnemonic { name:"SBC", opcode: 0xF1, addr_mode: AddressingMode::IndirectIndexed, ofunc: CPU::sbc },
    Mnemonic { name:"SBC", opcode: 0xF5, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::sbc },
    Mnemonic { name:"SBC", opcode: 0xF9, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::sbc },
    Mnemonic { name:"SBC", opcode: 0xFD, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::sbc },
    // MNEMONIC:INC
    Mnemonic { name:"INC", opcode: 0xE6, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::inc },
    Mnemonic { name:"INC", opcode: 0xEE, addr_mode: AddressingMode::Absolute, ofunc: CPU::inc },
    Mnemonic { name:"INC", opcode: 0xF6, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::inc },
    Mnemonic { name:"INC", opcode: 0xFE, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::inc },
    // MNEMONIC:INX
    Mnemonic { name:"INX", opcode: 0xE8, addr_mode: AddressingMode::Implied, ofunc: CPU::inx },
    // MNEMONIC:INY
    Mnemonic { name:"INY", opcode: 0xC8, addr_mode: AddressingMode::Implied, ofunc: CPU::iny },
    // MNEMONIC:DEC
    Mnemonic { name:"DEC", opcode: 0xC6, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::dec },
    Mnemonic { name:"DEC", opcode: 0xCE, addr_mode: AddressingMode::Absolute, ofunc: CPU::dec },
    Mnemonic { name:"DEC", opcode: 0xD6, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::dec },
    Mnemonic { name:"DEC", opcode: 0xDE, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::dec },
    // MNEMONIC:DEX
    Mnemonic { name:"DEX", opcode: 0xCA, addr_mode: AddressingMode::Implied, ofunc: CPU::dex },
    // MNEMONIC:DEY
    Mnemonic { name:"DEY", opcode: 0x88, addr_mode: AddressingMode::Implied, ofunc: CPU::dey },
    // MNEMONIC:AND
    Mnemonic { name:"AND", opcode: 0x21, addr_mode: AddressingMode::IndexedIndirect, ofunc: CPU::and },
    Mnemonic { name:"AND", opcode: 0x25, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::and },
    Mnemonic { name:"AND", opcode: 0x29, addr_mode: AddressingMode::Immediate, ofunc: CPU::and },
    Mnemonic { name:"AND", opcode: 0x2D, addr_mode: AddressingMode::Absolute, ofunc: CPU::and },
    Mnemonic { name:"AND", opcode: 0x31, addr_mode: AddressingMode::IndirectIndexed, ofunc: CPU::and },
    Mnemonic { name:"AND", opcode: 0x35, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::and },
    Mnemonic { name:"AND", opcode: 0x39, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::and },
    Mnemonic { name:"AND", opcode: 0x3D, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::and },
    // MNEMONIC:ORA
    Mnemonic { name:"ORA", opcode: 0x01, addr_mode: AddressingMode::IndexedIndirect, ofunc: CPU::ora },
    Mnemonic { name:"ORA", opcode: 0x05, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::ora },
    Mnemonic { name:"ORA", opcode: 0x09, addr_mode: AddressingMode::Immediate, ofunc: CPU::ora },
    Mnemonic { name:"ORA", opcode: 0x0D, addr_mode: AddressingMode::Absolute, ofunc: CPU::ora },
    Mnemonic { name:"ORA", opcode: 0x11, addr_mode: AddressingMode::IndirectIndexed, ofunc: CPU::ora },
    Mnemonic { name:"ORA", opcode: 0x15, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::ora },
    Mnemonic { name:"ORA", opcode: 0x19, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::ora },
    Mnemonic { name:"ORA", opcode: 0x1D, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::ora },
    // MNEMONIC:EOR
    Mnemonic { name:"EOR", opcode: 0x01, addr_mode: AddressingMode::IndexedIndirect, ofunc: CPU::eor },
    Mnemonic { name:"EOR", opcode: 0x05, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::eor },
    Mnemonic { name:"EOR", opcode: 0x09, addr_mode: AddressingMode::Immediate, ofunc: CPU::eor },
    Mnemonic { name:"EOR", opcode: 0x0D, addr_mode: AddressingMode::Absolute, ofunc: CPU::eor },
    Mnemonic { name:"EOR", opcode: 0x11, addr_mode: AddressingMode::IndirectIndexed, ofunc: CPU::eor },
    Mnemonic { name:"EOR", opcode: 0x15, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::eor },
    Mnemonic { name:"EOR", opcode: 0x19, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::eor },
    Mnemonic { name:"EOR", opcode: 0x1D, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::eor },
    // MNEMONIC:ASL
    Mnemonic { name:"ASL", opcode: 0x06, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::asl },
    Mnemonic { name:"ASL", opcode: 0x0A, addr_mode: AddressingMode::Accumulator, ofunc: CPU::asl },
    Mnemonic { name:"ASL", opcode: 0x0E, addr_mode: AddressingMode::Absolute, ofunc: CPU::asl },
    Mnemonic { name:"ASL", opcode: 0x16, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::asl },
    Mnemonic { name:"ASL", opcode: 0x1E, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::asl },
    // MNEMONIC:LSR
    Mnemonic { name:"LSR", opcode: 0x46, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::lsr },
    Mnemonic { name:"LSR", opcode: 0x4A, addr_mode: AddressingMode::Accumulator, ofunc: CPU::lsr },
    Mnemonic { name:"LSR", opcode: 0x4E, addr_mode: AddressingMode::Absolute, ofunc: CPU::lsr },
    Mnemonic { name:"LSR", opcode: 0x56, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::lsr },
    Mnemonic { name:"LSR", opcode: 0x5E, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::lsr },
    // MNEMONIC:ROL
    Mnemonic { name:"ROL", opcode: 0x26, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::rol },
    Mnemonic { name:"ROL", opcode: 0x2A, addr_mode: AddressingMode::Accumulator, ofunc: CPU::rol },
    Mnemonic { name:"ROL", opcode: 0x2E, addr_mode: AddressingMode::Absolute, ofunc: CPU::rol },
    Mnemonic { name:"ROL", opcode: 0x36, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::rol },
    Mnemonic { name:"ROL", opcode: 0x3E, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::rol },
    // MNEMONIC:ROR
    Mnemonic { name:"ROR", opcode: 0x66, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::ror },
    Mnemonic { name:"ROR", opcode: 0x6A, addr_mode: AddressingMode::Accumulator, ofunc: CPU::ror },
    Mnemonic { name:"ROR", opcode: 0x6E, addr_mode: AddressingMode::Absolute, ofunc: CPU::ror },
    Mnemonic { name:"ROR", opcode: 0x76, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::ror },
    Mnemonic { name:"ROR", opcode: 0x7E, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::ror },
    // MNEMONIC:CMP
    Mnemonic { name:"CMP", opcode: 0xC1, addr_mode: AddressingMode::IndexedIndirect, ofunc: CPU::cmp },
    Mnemonic { name:"CMP", opcode: 0xC5, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::cmp },
    Mnemonic { name:"CMP", opcode: 0xC9, addr_mode: AddressingMode::Immediate, ofunc: CPU::cmp },
    Mnemonic { name:"CMP", opcode: 0xCD, addr_mode: AddressingMode::Absolute, ofunc: CPU::cmp },
    Mnemonic { name:"CMP", opcode: 0xD1, addr_mode: AddressingMode::IndirectIndexed, ofunc: CPU::cmp },
    Mnemonic { name:"CMP", opcode: 0xD5, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::cmp },
    Mnemonic { name:"CMP", opcode: 0xD9, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::cmp },
    Mnemonic { name:"CMP", opcode: 0xDD, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::cmp },
    // MNEMONIC:CPX
    Mnemonic { name:"CPX", opcode: 0xE0, addr_mode: AddressingMode::Immediate, ofunc: CPU::cpx },
    Mnemonic { name:"CPX", opcode: 0xE4, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::cpx },
    Mnemonic { name:"CPX", opcode: 0xEC, addr_mode: AddressingMode::Absolute, ofunc: CPU::cpx },
    // MNEMONIC:CPY
    Mnemonic { name:"CPY", opcode: 0xC0, addr_mode: AddressingMode::Immediate, ofunc: CPU::cpy },
    Mnemonic { name:"CPY", opcode: 0xC4, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::cpy },
    Mnemonic { name:"CPY", opcode: 0xCC, addr_mode: AddressingMode::Absolute, ofunc: CPU::cpy },
    // MNEMONIC:BIT
    Mnemonic { name:"BIT", opcode: 0x24, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::bit },
    Mnemonic { name:"BIT", opcode: 0x2C, addr_mode: AddressingMode::Absolute, ofunc: CPU::bit },
    // MNEMONIC:LDA
    Mnemonic { name:"LDA", opcode: 0xA1, addr_mode: AddressingMode::IndexedIndirect, ofunc: CPU::lda },
    Mnemonic { name:"LDA", opcode: 0xA5, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::lda },
    Mnemonic { name:"LDA", opcode: 0xA9, addr_mode: AddressingMode::Immediate, ofunc: CPU::lda },
    Mnemonic { name:"LDA", opcode: 0xAD, addr_mode: AddressingMode::Absolute, ofunc: CPU::lda },
    Mnemonic { name:"LDA", opcode: 0xB1, addr_mode: AddressingMode::IndirectIndexed, ofunc: CPU::lda },
    Mnemonic { name:"LDA", opcode: 0xB5, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::lda },
    Mnemonic { name:"LDA", opcode: 0xB9, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::lda },
    Mnemonic { name:"LDA", opcode: 0xBD, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::lda },
    // MNEMONIC:LDX
    Mnemonic { name:"LDX", opcode: 0xA2, addr_mode: AddressingMode::Immediate, ofunc: CPU::ldx },
    Mnemonic { name:"LDX", opcode: 0xA6, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::ldx },
    Mnemonic { name:"LDX", opcode: 0xAE, addr_mode: AddressingMode::Absolute, ofunc: CPU::ldx },
    Mnemonic { name:"LDX", opcode: 0xB6, addr_mode: AddressingMode::ZeroPageIndexedY, ofunc: CPU::ldx },
    Mnemonic { name:"LDX", opcode: 0xBE, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::ldx },
    // MNEMONIC:LDY
    Mnemonic { name:"LDY", opcode: 0xA0, addr_mode: AddressingMode::Immediate, ofunc: CPU::ldy },
    Mnemonic { name:"LDY", opcode: 0xA4, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::ldy },
    Mnemonic { name:"LDY", opcode: 0xAC, addr_mode: AddressingMode::Absolute, ofunc: CPU::ldy },
    Mnemonic { name:"LDY", opcode: 0xB4, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::ldy },
    Mnemonic { name:"LDY", opcode: 0xBC, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::ldy },
    // MNEMONIC:STA
    Mnemonic { name:"STA", opcode: 0x81, addr_mode: AddressingMode::IndexedIndirect, ofunc: CPU::sta },
    Mnemonic { name:"STA", opcode: 0x85, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::sta },
    Mnemonic { name:"STA", opcode: 0x8D, addr_mode: AddressingMode::Absolute, ofunc: CPU::sta },
    Mnemonic { name:"STA", opcode: 0x91, addr_mode: AddressingMode::IndirectIndexed, ofunc: CPU::sta },
    Mnemonic { name:"STA", opcode: 0x95, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::sta },
    Mnemonic { name:"STA", opcode: 0x99, addr_mode: AddressingMode::AbsoluteIndexedY, ofunc: CPU::sta },
    Mnemonic { name:"STA", opcode: 0x9D, addr_mode: AddressingMode::AbsoluteIndexedX, ofunc: CPU::sta },
    // MNEMONIC:STX
    Mnemonic { name:"STX", opcode: 0x86, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::stx },
    Mnemonic { name:"STX", opcode: 0x8E, addr_mode: AddressingMode::Absolute, ofunc: CPU::stx },
    Mnemonic { name:"STX", opcode: 0x96, addr_mode: AddressingMode::ZeroPageIndexedY, ofunc: CPU::stx },
    // MNEMONIC:STY
    Mnemonic { name:"STY", opcode: 0x84, addr_mode: AddressingMode::ZeroPage, ofunc: CPU::sty },
    Mnemonic { name:"STY", opcode: 0x8C, addr_mode: AddressingMode::Absolute, ofunc: CPU::sty },
    Mnemonic { name:"STY", opcode: 0x94, addr_mode: AddressingMode::ZeroPageIndexedX, ofunc: CPU::sty },
    // MNEMONIC:BCC
    Mnemonic { name:"BCC", opcode: 0x90, addr_mode: AddressingMode::Relative, ofunc: CPU::bcc },
    // MNEMONIC:BCS
    Mnemonic { name:"BCS", opcode: 0xB0, addr_mode: AddressingMode::Relative, ofunc: CPU::bcs },
    // MNEMONIC:BNE
    Mnemonic { name:"BNE", opcode: 0xD0, addr_mode: AddressingMode::Relative, ofunc: CPU::bne },
    // MNEMONIC:BEQ
    Mnemonic { name:"BEQ", opcode: 0xF0, addr_mode: AddressingMode::Relative, ofunc: CPU::beq },
    // MNEMONIC:BPL
    Mnemonic { name:"BPL", opcode: 0x10, addr_mode: AddressingMode::Relative, ofunc: CPU::bpl },
    // MNEMONIC:BMI
    Mnemonic { name:"BMI", opcode: 0x30, addr_mode: AddressingMode::Relative, ofunc: CPU::bmi },
    // MNEMONIC:BVC
    Mnemonic { name:"BVC", opcode: 0x90, addr_mode: AddressingMode::Relative, ofunc: CPU::bvc },
    // MNEMONIC:BVS
    Mnemonic { name:"BVS", opcode: 0x90, addr_mode: AddressingMode::Relative, ofunc: CPU::bvs },
    // MNEMONIC:TAX
    Mnemonic { name:"TAX", opcode: 0xAA, addr_mode: AddressingMode::Implied, ofunc: CPU::tax },
    // MNEMONIC:TXA
    Mnemonic { name:"TXA", opcode: 0x8A, addr_mode: AddressingMode::Implied, ofunc: CPU::txa },
    // MNEMONIC:TAY
    Mnemonic { name:"TAY", opcode: 0xA8, addr_mode: AddressingMode::Implied, ofunc: CPU::tay },
    // MNEMONIC:TYA
    Mnemonic { name:"TYA", opcode: 0x98, addr_mode: AddressingMode::Implied, ofunc: CPU::tya },
    // MNEMONIC:TSX
    Mnemonic { name:"TSX", opcode: 0xBA, addr_mode: AddressingMode::Implied, ofunc: CPU::tsx },
    // MNEMONIC:TXS
    Mnemonic { name:"TXS", opcode: 0x9A, addr_mode: AddressingMode::Implied, ofunc: CPU::txs },
    // MNEMONIC:PHA
    Mnemonic { name:"PHA", opcode: 0x48, addr_mode: AddressingMode::Implied, ofunc: CPU::pha },
    // MNEMONIC:PLA
    Mnemonic { name:"PLA", opcode: 0x68, addr_mode: AddressingMode::Implied, ofunc: CPU::pla },
    // MNEMONIC:PHP
    Mnemonic { name:"PHP", opcode: 0x08, addr_mode: AddressingMode::Implied, ofunc: CPU::php },
    // MNEMONIC:PLP
    Mnemonic { name:"PLP", opcode: 0x28, addr_mode: AddressingMode::Implied, ofunc: CPU::plp },
    // MNEMONIC:CLC
    Mnemonic { name:"CLC", opcode: 0x18, addr_mode: AddressingMode::Implied, ofunc: CPU::clc },
    // MNEMONIC:SEC
    Mnemonic { name:"SEC", opcode: 0x38, addr_mode: AddressingMode::Implied, ofunc: CPU::sec },
    // MNEMONIC:CLD
    Mnemonic { name:"CLD", opcode: 0xD8, addr_mode: AddressingMode::Implied, ofunc: CPU::cld },
    // MNEMONIC:SED
    Mnemonic { name:"SED", opcode: 0xF8, addr_mode: AddressingMode::Implied, ofunc: CPU::sed },
    // MNEMONIC:CLI
    Mnemonic { name:"CLI", opcode: 0x58, addr_mode: AddressingMode::Implied, ofunc: CPU::cli },
    // MNEMONIC:SEI
    Mnemonic { name:"SEI", opcode: 0x78, addr_mode: AddressingMode::Implied, ofunc: CPU::sei },
    // MNEMONIC:CLV
    Mnemonic { name:"CLV", opcode: 0xB8, addr_mode: AddressingMode::Implied, ofunc: CPU::clv },
    ];