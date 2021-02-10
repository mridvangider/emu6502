use super::util::{
    Mnemonic,
    AddressingMode
};
use super::arithmetic::{
    add,
    sub,
    inc
};

const mnemomics: [Mnemonic;20] = [ 
    Mnemonic { name:"ADC", opcode: 0x61, addr_mode: AddressingMode::IndexedIndirect, func: add},
    Mnemonic { name:"ADC", opcode: 0x65, addr_mode: AddressingMode::ZeroPage, func: add },
    Mnemonic { name:"ADC", opcode: 0x69, addr_mode: AddressingMode::Immediate, func: add},
    Mnemonic { name:"ADC", opcode: 0x6D, addr_mode: AddressingMode::Absolute, func: add},
    Mnemonic { name:"ADC", opcode: 0x71, addr_mode: AddressingMode::IndirectIndexed, func: add},
    Mnemonic { name:"ADC", opcode: 0x75, addr_mode: AddressingMode::ZeroPageIndexedX, func: add},
    Mnemonic { name:"ADC", opcode: 0x79, addr_mode: AddressingMode::AbsoluteIndexedY, func: add},
    Mnemonic { name:"ADC", opcode: 0x7D, addr_mode: AddressingMode::AbsoluteIndexedX, func: add},
    
    Mnemonic { name:"SBC", opcode: 0xE1, addr_mode: AddressingMode::IndexedIndirect, func: sub},
    Mnemonic { name:"SBC", opcode: 0xE5, addr_mode: AddressingMode::ZeroPage, func: sub},
    Mnemonic { name:"SBC", opcode: 0xE9, addr_mode: AddressingMode::Immediate, func: sub},
    Mnemonic { name:"SBC", opcode: 0xED, addr_mode: AddressingMode::Absolute, func: sub},
    Mnemonic { name:"SBC", opcode: 0xF1, addr_mode: AddressingMode::IndirectIndexed, func: sub},
    Mnemonic { name:"SBC", opcode: 0xF5, addr_mode: AddressingMode::ZeroPageIndexedX, func: sub},
    Mnemonic { name:"SBC", opcode: 0xF9, addr_mode: AddressingMode::AbsoluteIndexedY, func: sub},
    Mnemonic { name:"SBC", opcode: 0xFD, addr_mode: AddressingMode::AbsoluteIndexedX, func: sub},

    Mnemonic { name:"INC", opcode: 0xE6, addr_mode: AddressingMode::ZeroPage, func: inc},
    Mnemonic { name:"INC", opcode: 0xEE, addr_mode: AddressingMode::ZeroPage, func: inc},
    Mnemonic { name:"INC", opcode: 0xF6, addr_mode: AddressingMode::ZeroPage, func: inc},
    Mnemonic { name:"INC", opcode: 0xFE, addr_mode: AddressingMode::ZeroPage, func: inc},
    ];