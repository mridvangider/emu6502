pub type Memory = Vec<u8>;
pub type OperationFunction = fn (&mut u8, &mut u8, &mut u8, &mut u16, &mut u8, &mut u8, &Operand, &mut Memory, mode: AddressingMode);

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

pub const FLAG_CARRY: u8 = 0x01;    // 00 00 00 01
pub const FLAG_ZERO: u8 = 0x02;     // 00 00 00 10
pub const FLAG_INTDIS: u8 = 0x04;   // 00 00 01 00
pub const FLAG_DECIMAL: u8 = 0x08;  // 00 00 10 00
pub const FLAG_BREAK: u8 = 0x10;    // 00 01 00 00
pub const FLAG_OVERFLOW: u8 = 0x40; // 01 00 00 00
pub const FLAG_NEGATIVE: u8 = 0x80; // 10 00 00 00

pub enum Operand {
    Null,
    Byte(u8),
    Word(u16)
}

#[derive(PartialEq)]
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
    pub name:       &'a str,
    pub opcode:     u8,
    pub addr_mode:  AddressingMode,
    pub func:       OperationFunction
}

pub struct Instruction {
    pub opcode:         u8,
    pub operands:       std::vec::Vec<Operand>,
}

pub fn change_endianness(val: &u16) -> u16 {
    let mut ret: u16;
    ret = val & 0x00FF;
    ret  = ret << 8;
    ret |= val >> 8;
    return ret;
}

pub fn split_word(val: &u16) -> [u8;2] {
    let mut ret = [0,0];
    ret[0] = ( val & 0x00FF ) as u8;
    ret[1] = ( ( val & 0xFF00 ) >> 8 ) as u8;
    return ret;
}

pub fn calculate_address(op: &Operand, reg_x: &u8, reg_y: &u8, pc: &u16, mem: &Memory, mode: AddressingMode) -> Option<usize> {
    let mut ret: usize;
    match mode {
        AddressingMode::Absolute => {
            if let Operand::Word(addr) = op {
                return Some(change_endianness(addr) as usize);
            } else {
                return None;
            }
        },
        AddressingMode::AbsoluteIndexedX => {
            if let Operand::Word(addr) = op {
                ret = change_endianness(addr) as usize;
                ret += *reg_x as usize;
                return Some(ret);
            } else {
                return None;
            }
        },
        AddressingMode::AbsoluteIndexedY => {
            if let Operand::Word(addr) = op {
                ret = change_endianness(addr) as usize;
                ret += *reg_y as usize;
                return Some(ret);
            } else {
                return None;
            }
        },
        AddressingMode::Indirect => {
            if let Operand::Word(addr) = op {
                ret = change_endianness(addr) as usize;
                ret = mem[ret] as usize ;
                return Some(ret);
            } else {
                return None;
            }
        },
        AddressingMode::IndexedIndirect => {
            if let Operand::Byte(addr) = op {
                ret = ( *addr as usize ) + ( *reg_x as usize );
                ret = mem[ret] as usize;
                return Some(ret);
            } else {
                return None;
            }
        },
        AddressingMode::IndirectIndexed => {
            if let Operand::Byte(addr) = op {
                ret = ( mem[*addr as usize] + *reg_y ) as usize;
                return Some(ret);
            } else {
                 return None;
            }
        },
        AddressingMode::Relative => {
            if let Operand::Byte(addr) = op {
                ret = ( ( *pc as i32) + ( *addr as i32 ) ) as usize;
                return Some(ret);
            } else {
                return None;
            }
        },
        AddressingMode::ZeroPageIndexedX => {
            if let Operand::Byte(addr) = op {
                ret = ( ( *addr + *reg_x) % 0xff ) as usize;
                return Some(ret);
            } else {
                return None;
            }
        },
        AddressingMode::ZeroPageIndexedY => {
            if let Operand::Byte(addr) = op {
                ret = ( ( *addr + *reg_y) % 0xff ) as usize;
                return Some(ret);
            } else {
                return None;
            }
        }
        _ => return None
    }
}