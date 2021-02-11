use super::util::{
    AddressingMode,
    Operand,
    Memory,
    FLAG_ZERO,
    FLAG_NEGATIVE
};

pub fn tax(reg_a: &mut u8, reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *reg_x = *reg_a;

        if *reg_x == 0 {
            *stat |= FLAG_ZERO;
        }

        *stat |= FLAG_NEGATIVE & *reg_x;
    }
}

pub fn txa(reg_a: &mut u8, reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *reg_a = *reg_x;
        
        if *reg_a == 0 {
            *stat |= FLAG_ZERO;
        }

        *stat |= FLAG_NEGATIVE & *reg_a;
    }
}

pub fn tay(reg_a: &mut u8, _reg_x: &mut u8, reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *reg_y = *reg_a;

        if *reg_y == 0 {
            *stat |= FLAG_ZERO;
        }

        *stat |= FLAG_NEGATIVE & *reg_y;
    }
}

pub fn tya(reg_a: &mut u8, _reg_x: &mut u8, reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *reg_a = *reg_y;
        
        if *reg_a == 0 {
            *stat |= FLAG_ZERO;
        }

        *stat |= FLAG_NEGATIVE & *reg_a;
    }
}

pub fn tsx(_reg_a: &mut u8, reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *reg_x = *sp;

        if *reg_x == 0 {
            *stat |= FLAG_ZERO;
        }

        *stat |= FLAG_NEGATIVE & *reg_x;
    }
}

pub fn txs(_reg_a: &mut u8, reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, sp: &mut u8, _stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *sp = *reg_x;
    }
}