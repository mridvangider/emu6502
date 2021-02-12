use super::util::{
    AddressingMode,
    Operand,
    Memory,
    FLAG_CARRY,
    FLAG_DECIMAL,
    FLAG_INTDIS,
    FLAG_OVERFLOW,
};

pub fn clc (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *stat &= !FLAG_CARRY;
    }
}

pub fn sec (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *stat |= FLAG_CARRY;
    }
}

pub fn cld (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *stat &= !FLAG_DECIMAL;
    }
}

pub fn sed (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *stat |= FLAG_DECIMAL;
    }
}

pub fn cli (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *stat &= !FLAG_INTDIS;
    }
}

pub fn sei (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *stat |= FLAG_INTDIS;
    }
}

pub fn clv (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *stat &= !FLAG_OVERFLOW;
    }
}