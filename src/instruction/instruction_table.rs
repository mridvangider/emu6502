use super::super::util::{
    Mnemonic,
    AddressingMode,
};

pub const EMPTY_MNEM : Mnemonic = Mnemonic { name:"NON", opcode: 0x00, addr_mode: AddressingMode::Implied };

pub const MNEMONICS: [Mnemonic;144] = [
    // MNEMONIC:ADC
    Mnemonic { name:"ADC", opcode: 0x61, addr_mode: AddressingMode::IndexedIndirect },
    Mnemonic { name:"ADC", opcode: 0x65, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"ADC", opcode: 0x69, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"ADC", opcode: 0x6D, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"ADC", opcode: 0x71, addr_mode: AddressingMode::IndirectIndexed },
    Mnemonic { name:"ADC", opcode: 0x75, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"ADC", opcode: 0x79, addr_mode: AddressingMode::AbsoluteIndexedY },
    Mnemonic { name:"ADC", opcode: 0x7D, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:SBC
    Mnemonic { name:"SBC", opcode: 0xE1, addr_mode: AddressingMode::IndexedIndirect },
    Mnemonic { name:"SBC", opcode: 0xE5, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"SBC", opcode: 0xE9, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"SBC", opcode: 0xED, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"SBC", opcode: 0xF1, addr_mode: AddressingMode::IndirectIndexed },
    Mnemonic { name:"SBC", opcode: 0xF5, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"SBC", opcode: 0xF9, addr_mode: AddressingMode::AbsoluteIndexedY },
    Mnemonic { name:"SBC", opcode: 0xFD, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:INC
    Mnemonic { name:"INC", opcode: 0xE6, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"INC", opcode: 0xEE, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"INC", opcode: 0xF6, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"INC", opcode: 0xFE, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:INX
    Mnemonic { name:"INX", opcode: 0xE8, addr_mode: AddressingMode::Implied },
    // MNEMONIC:INY
    Mnemonic { name:"INY", opcode: 0xC8, addr_mode: AddressingMode::Implied },
    // MNEMONIC:DEC
    Mnemonic { name:"DEC", opcode: 0xC6, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"DEC", opcode: 0xCE, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"DEC", opcode: 0xD6, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"DEC", opcode: 0xDE, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:DEX
    Mnemonic { name:"DEX", opcode: 0xCA, addr_mode: AddressingMode::Implied },
    // MNEMONIC:DEY
    Mnemonic { name:"DEY", opcode: 0x88, addr_mode: AddressingMode::Implied },
    // MNEMONIC:AND
    Mnemonic { name:"AND", opcode: 0x21, addr_mode: AddressingMode::IndexedIndirect },
    Mnemonic { name:"AND", opcode: 0x25, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"AND", opcode: 0x29, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"AND", opcode: 0x2D, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"AND", opcode: 0x31, addr_mode: AddressingMode::IndirectIndexed },
    Mnemonic { name:"AND", opcode: 0x35, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"AND", opcode: 0x39, addr_mode: AddressingMode::AbsoluteIndexedY },
    Mnemonic { name:"AND", opcode: 0x3D, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:ORA
    Mnemonic { name:"ORA", opcode: 0x01, addr_mode: AddressingMode::IndexedIndirect },
    Mnemonic { name:"ORA", opcode: 0x05, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"ORA", opcode: 0x09, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"ORA", opcode: 0x0D, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"ORA", opcode: 0x11, addr_mode: AddressingMode::IndirectIndexed },
    Mnemonic { name:"ORA", opcode: 0x15, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"ORA", opcode: 0x19, addr_mode: AddressingMode::AbsoluteIndexedY },
    Mnemonic { name:"ORA", opcode: 0x1D, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:EOR
    Mnemonic { name:"EOR", opcode: 0x01, addr_mode: AddressingMode::IndexedIndirect },
    Mnemonic { name:"EOR", opcode: 0x05, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"EOR", opcode: 0x09, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"EOR", opcode: 0x0D, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"EOR", opcode: 0x11, addr_mode: AddressingMode::IndirectIndexed },
    Mnemonic { name:"EOR", opcode: 0x15, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"EOR", opcode: 0x19, addr_mode: AddressingMode::AbsoluteIndexedY },
    Mnemonic { name:"EOR", opcode: 0x1D, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:ASL
    Mnemonic { name:"ASL", opcode: 0x06, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"ASL", opcode: 0x0A, addr_mode: AddressingMode::Accumulator },
    Mnemonic { name:"ASL", opcode: 0x0E, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"ASL", opcode: 0x16, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"ASL", opcode: 0x1E, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:LSR
    Mnemonic { name:"LSR", opcode: 0x46, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"LSR", opcode: 0x4A, addr_mode: AddressingMode::Accumulator },
    Mnemonic { name:"LSR", opcode: 0x4E, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"LSR", opcode: 0x56, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"LSR", opcode: 0x5E, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:ROL
    Mnemonic { name:"ROL", opcode: 0x26, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"ROL", opcode: 0x2A, addr_mode: AddressingMode::Accumulator },
    Mnemonic { name:"ROL", opcode: 0x2E, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"ROL", opcode: 0x36, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"ROL", opcode: 0x3E, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:ROR
    Mnemonic { name:"ROR", opcode: 0x66, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"ROR", opcode: 0x6A, addr_mode: AddressingMode::Accumulator },
    Mnemonic { name:"ROR", opcode: 0x6E, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"ROR", opcode: 0x76, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"ROR", opcode: 0x7E, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:CMP
    Mnemonic { name:"CMP", opcode: 0xC1, addr_mode: AddressingMode::IndexedIndirect },
    Mnemonic { name:"CMP", opcode: 0xC5, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"CMP", opcode: 0xC9, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"CMP", opcode: 0xCD, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"CMP", opcode: 0xD1, addr_mode: AddressingMode::IndirectIndexed },
    Mnemonic { name:"CMP", opcode: 0xD5, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"CMP", opcode: 0xD9, addr_mode: AddressingMode::AbsoluteIndexedY },
    Mnemonic { name:"CMP", opcode: 0xDD, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:CPX
    Mnemonic { name:"CPX", opcode: 0xE0, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"CPX", opcode: 0xE4, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"CPX", opcode: 0xEC, addr_mode: AddressingMode::Absolute },
    // MNEMONIC:CPY
    Mnemonic { name:"CPY", opcode: 0xC0, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"CPY", opcode: 0xC4, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"CPY", opcode: 0xCC, addr_mode: AddressingMode::Absolute },
    // MNEMONIC:BIT
    Mnemonic { name:"BIT", opcode: 0x24, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"BIT", opcode: 0x2C, addr_mode: AddressingMode::Absolute },
    // MNEMONIC:LDA
    Mnemonic { name:"LDA", opcode: 0xA1, addr_mode: AddressingMode::IndexedIndirect },
    Mnemonic { name:"LDA", opcode: 0xA5, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"LDA", opcode: 0xA9, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"LDA", opcode: 0xAD, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"LDA", opcode: 0xB1, addr_mode: AddressingMode::IndirectIndexed },
    Mnemonic { name:"LDA", opcode: 0xB5, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"LDA", opcode: 0xB9, addr_mode: AddressingMode::AbsoluteIndexedY },
    Mnemonic { name:"LDA", opcode: 0xBD, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:LDX
    Mnemonic { name:"LDX", opcode: 0xA2, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"LDX", opcode: 0xA6, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"LDX", opcode: 0xAE, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"LDX", opcode: 0xB6, addr_mode: AddressingMode::ZeroPageIndexedY },
    Mnemonic { name:"LDX", opcode: 0xBE, addr_mode: AddressingMode::AbsoluteIndexedY },
    // MNEMONIC:LDY
    Mnemonic { name:"LDY", opcode: 0xA0, addr_mode: AddressingMode::Immediate },
    Mnemonic { name:"LDY", opcode: 0xA4, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"LDY", opcode: 0xAC, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"LDY", opcode: 0xB4, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"LDY", opcode: 0xBC, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:STA
    Mnemonic { name:"STA", opcode: 0x81, addr_mode: AddressingMode::IndexedIndirect },
    Mnemonic { name:"STA", opcode: 0x85, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"STA", opcode: 0x8D, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"STA", opcode: 0x91, addr_mode: AddressingMode::IndirectIndexed },
    Mnemonic { name:"STA", opcode: 0x95, addr_mode: AddressingMode::ZeroPageIndexedX },
    Mnemonic { name:"STA", opcode: 0x99, addr_mode: AddressingMode::AbsoluteIndexedY },
    Mnemonic { name:"STA", opcode: 0x9D, addr_mode: AddressingMode::AbsoluteIndexedX },
    // MNEMONIC:STX
    Mnemonic { name:"STX", opcode: 0x86, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"STX", opcode: 0x8E, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"STX", opcode: 0x96, addr_mode: AddressingMode::ZeroPageIndexedY },
    // MNEMONIC:STY
    Mnemonic { name:"STY", opcode: 0x84, addr_mode: AddressingMode::ZeroPage },
    Mnemonic { name:"STY", opcode: 0x8C, addr_mode: AddressingMode::Absolute },
    Mnemonic { name:"STY", opcode: 0x94, addr_mode: AddressingMode::ZeroPageIndexedX },
    // MNEMONIC:BCC
    Mnemonic { name:"BCC", opcode: 0x90, addr_mode: AddressingMode::Relative },
    // MNEMONIC:BCS
    Mnemonic { name:"BCS", opcode: 0xB0, addr_mode: AddressingMode::Relative },
    // MNEMONIC:BNE
    Mnemonic { name:"BNE", opcode: 0xD0, addr_mode: AddressingMode::Relative },
    // MNEMONIC:BEQ
    Mnemonic { name:"BEQ", opcode: 0xF0, addr_mode: AddressingMode::Relative },
    // MNEMONIC:BPL
    Mnemonic { name:"BPL", opcode: 0x10, addr_mode: AddressingMode::Relative },
    // MNEMONIC:BMI
    Mnemonic { name:"BMI", opcode: 0x30, addr_mode: AddressingMode::Relative },
    // MNEMONIC:BVC
    Mnemonic { name:"BVC", opcode: 0x90, addr_mode: AddressingMode::Relative },
    // MNEMONIC:BVS
    Mnemonic { name:"BVS", opcode: 0x90, addr_mode: AddressingMode::Relative },
    // MNEMONIC:TAX
    Mnemonic { name:"TAX", opcode: 0xAA, addr_mode: AddressingMode::Implied },
    // MNEMONIC:TXA
    Mnemonic { name:"TXA", opcode: 0x8A, addr_mode: AddressingMode::Implied },
    // MNEMONIC:TAY
    Mnemonic { name:"TAY", opcode: 0xA8, addr_mode: AddressingMode::Implied },
    // MNEMONIC:TYA
    Mnemonic { name:"TYA", opcode: 0x98, addr_mode: AddressingMode::Implied },
    // MNEMONIC:TSX
    Mnemonic { name:"TSX", opcode: 0xBA, addr_mode: AddressingMode::Implied },
    // MNEMONIC:TXS
    Mnemonic { name:"TXS", opcode: 0x9A, addr_mode: AddressingMode::Implied },
    // MNEMONIC:PHA
    Mnemonic { name:"PHA", opcode: 0x48, addr_mode: AddressingMode::Implied },
    // MNEMONIC:PLA
    Mnemonic { name:"PLA", opcode: 0x68, addr_mode: AddressingMode::Implied },
    // MNEMONIC:PHP
    Mnemonic { name:"PHP", opcode: 0x08, addr_mode: AddressingMode::Implied },
    // MNEMONIC:PLP
    Mnemonic { name:"PLP", opcode: 0x28, addr_mode: AddressingMode::Implied },
    // MNEMONIC:CLC
    Mnemonic { name:"CLC", opcode: 0x18, addr_mode: AddressingMode::Implied },
    // MNEMONIC:SEC
    Mnemonic { name:"SEC", opcode: 0x38, addr_mode: AddressingMode::Implied },
    // MNEMONIC:CLD
    Mnemonic { name:"CLD", opcode: 0xD8, addr_mode: AddressingMode::Implied },
    // MNEMONIC:SED
    Mnemonic { name:"SED", opcode: 0xF8, addr_mode: AddressingMode::Implied },
    // MNEMONIC:CLI
    Mnemonic { name:"CLI", opcode: 0x58, addr_mode: AddressingMode::Implied },
    // MNEMONIC:SEI
    Mnemonic { name:"SEI", opcode: 0x78, addr_mode: AddressingMode::Implied },
    // MNEMONIC:CLV
    Mnemonic { name:"CLV", opcode: 0xB8, addr_mode: AddressingMode::Implied },
    ];