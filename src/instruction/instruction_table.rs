use super::util::{
    Mnemonic,
    AddressingMode
};
use super::arithmetic::{
    add,
};

const mnenomics: [Mnemonic;8] = [ 
    Mnemonic { name:"ADC", opcode: 0x61, addr_mode: AddressingMode::IndexedIndirect, func: add},
    Mnemonic { name:"ADC", opcode: 0x65, addr_mode: AddressingMode::ZeroPage, func: add },
    Mnemonic { name:"ADC", opcode: 0x69, addr_mode: AddressingMode::Immediate, func: add},
    Mnemonic { name:"ADC", opcode: 0x6D, addr_mode: AddressingMode::Absolute, func: add},
    Mnemonic { name:"ADC", opcode: 0x71, addr_mode: AddressingMode::IndirectIndexed, func: add},
    Mnemonic { name:"ADC", opcode: 0x75, addr_mode: AddressingMode::ZeroPageIndexedX, func: add},
    Mnemonic { name:"ADC", opcode: 0x79, addr_mode: AddressingMode::AbsoluteIndexedY, func: add},
    Mnemonic { name:"ADC", opcode: 0x7D, addr_mode: AddressingMode::AbsoluteIndexedX, func: add},
    ];