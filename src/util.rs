use super::errors::*;
use super::cpu::CPU;

pub type Memory = Vec<u8>;
pub type OperationFunction = fn (&mut CPU, &Operand, mode: AddressingMode);

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

#[derive(PartialEq, Copy, Clone)]
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

pub type opfunc = fn(&mut CPU, &Operand, AddressingMode);

pub struct Mnemonic {
    pub name:       &'static str,
    pub opcode:     u8,
    pub addr_mode:  AddressingMode,
    pub ofunc:      opfunc,
}

pub struct Instruction<'a> {
    pub mnemonic:      &'a Mnemonic,
    pub operand:       Operand,
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

pub fn make_word(low: &u8, high: &u8) -> u16 {
    let mut ret: u16 = 0;
    ret |= *high as u16;
    ret = ret << 8;
    ret |= *low as u16;
    return ret;
}

pub fn safe_remove<T>(vec :&mut Vec<T>, index :usize) -> Result<T, Err> {
    if index < vec.len() {
        return Ok(vec.remove(index));
    } else {
        return Err(ERR_INVALID_INDEX);
    }
}