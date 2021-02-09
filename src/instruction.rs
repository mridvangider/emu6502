type Memory = Vec<u8>;
type OperationFunction = fn (&mut u8, &mut u8, &mut u8, &mut u16, &mut u8, &mut u8, &[Operand], &Memory, mode: AddressingMode);

#[derive(PartialEq)]
pub enum OperandType {
    Register,
    Memory,
    ZeroPage,
    Immediate
}

pub const REG_A: u8 = 0x01;
pub const REG_X: u8 = 0x02;
pub const REG_Y: u8 = 0x04;

pub struct Operand {
    operand_type:   OperandType,
    value:          [u8;2]
}

pub enum AddressingMode {
    Accumulator,
    Immediate,
    Implied,
    Relative,
    Absolute,
    ZeroPage,
    Indirect,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    IndexedIndirect,
    IndirectIndexed,
}

pub struct Mnemonic<'a> {
    name:       &'a str,
    opcode:     u8,
    addr_mode:  AddressingMode,
    func:       OperationFunction
}

pub struct Instruction {
    opcode:         u8,
    operands:       std::vec::Vec<Operand>,
}

pub fn parse_inst(data:&Vec<u8>) -> Option<Instruction> {
    return None;
}

const mnenomics: [Mnemonic;8] = [ 
    Mnemonic { name:"ADC", opcode: 0x61, addr_mode: AddressingMode::IndexedIndirect, func: Add},
    Mnemonic { name:"ADC", opcode: 0x65, addr_mode: AddressingMode::ZeroPage, func: Add },
    Mnemonic { name:"ADC", opcode: 0x69, addr_mode: AddressingMode::Immediate, func: Add},
    Mnemonic { name:"ADC", opcode: 0x6D, addr_mode: AddressingMode::Absolute, func: Add},
    Mnemonic { name:"ADC", opcode: 0x71, addr_mode: AddressingMode::IndirectIndexed, func: Add},
    Mnemonic { name:"ADC", opcode: 0x75, addr_mode: AddressingMode::ZeroPageIndexedX, func: Add},
    Mnemonic { name:"ADC", opcode: 0x79, addr_mode: AddressingMode::AbsoluteIndexedY, func: Add},
    Mnemonic { name:"ADC", opcode: 0x7D, addr_mode: AddressingMode::AbsoluteIndexedX, func: Add},
    ];


pub fn Add(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, sp: &mut u8, stat: &mut u8, ops: &[Operand], mem: &Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if ops[0].operand_type == OperandType::Immediate {
                *reg_a += ops[0].value[0];
            }
        },
        AddressingMode::Absolute => {
            if ops[0].operand_type == OperandType::Memory {
                let addr = ( ( ops[0].value[1] as u16 ) * 256 + ( ops[0].value[0] as u16 ) ) as usize;
                *reg_a += mem[addr];
            }
        },
        AddressingMode::ZeroPage => {
            if ops[0].operand_type == OperandType::ZeroPage {
                let addr = ops[0].value[0] as usize;
                *reg_a += mem[addr];
            }
        },
        AddressingMode::AbsoluteIndexedX => {
            if ops[0].operand_type == OperandType::Memory {
                let addr = ( ( ops[0].value[1] as u16 ) * 256 + ( ops[0].value[0] as u16 ) + ( *reg_x as u16 ) ) as usize;
                *reg_a += mem[addr]; 
            }
        },
        AddressingMode::AbsoluteIndexedY => {
            if ops[0].operand_type == OperandType::Memory {
                let addr = ( ( ops[0].value[1] as u16 ) * 256 + ( ops[0].value[0] as u16 ) + ( *reg_y as u16 ) ) as usize;
                *reg_a += mem[addr]; 
            }
        },
        AddressingMode::ZeroPageIndexedX => {
            if ops[0].operand_type == OperandType::ZeroPage {
                let addr = ( ( ops[0].value[0] as usize ) + ( *reg_x as usize ) ) % 0xff;
                *reg_a += mem[addr];
            }
        },
        AddressingMode::IndexedIndirect => {
            if ops[0].operand_type == OperandType::ZeroPage {
                let mut addr =  ( ( ops[0].value[0] as usize ) + ( *reg_x as usize ) ) % 0xff;
                addr = ( mem[addr+1] as usize ) * 256 + ( mem[addr] as usize);
                *reg_a += mem[addr];
            }
        },
        AddressingMode::IndirectIndexed => {
            if ops[0].operand_type == OperandType::ZeroPage {
                let mut addr = ops[0].value[0] as usize;
                addr = ( mem[addr] as usize ) + ( *reg_y as usize );
                *reg_a += mem[addr];
            }
        },
        _ => {}
    }
}