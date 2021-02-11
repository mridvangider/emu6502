use super::util::{
    AddressingMode,
    Operand,
    Memory,
    calculate_address,
    FLAG_CARRY,
    FLAG_ZERO,
    FLAG_NEGATIVE,
    FLAG_OVERFLOW
};

pub fn bcc(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if *stat & FLAG_CARRY == 0 {
        if mode == AddressingMode::Relative {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *pc = addr as u16;
            }
        }
    }
}

pub fn bcs(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if *stat & FLAG_CARRY == FLAG_CARRY {
        if mode == AddressingMode::Relative {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *pc = addr as u16;
            }
        }
    }
}

pub fn bne(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if *stat & FLAG_ZERO == 0 {
        if mode == AddressingMode::Relative {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *pc = addr as u16;
            }
        }
    }
}

pub fn beq(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if *stat & FLAG_ZERO == FLAG_ZERO {
        if mode == AddressingMode::Relative {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *pc = addr as u16;
            }
        }
    }
}

pub fn bpl(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if *stat & FLAG_NEGATIVE == 0 {
        if mode == AddressingMode::Relative {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *pc = addr as u16;
            }
        }
    }
}

pub fn bmi(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if *stat & FLAG_NEGATIVE == FLAG_NEGATIVE {
        if mode == AddressingMode::Relative {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *pc = addr as u16;
            }
        }
    }
}

pub fn bvc(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if *stat & FLAG_OVERFLOW == 0 {
        if mode == AddressingMode::Relative {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *pc = addr as u16;
            }
        }
    }
}

pub fn bvs(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if *stat & FLAG_OVERFLOW == FLAG_OVERFLOW {
        if mode == AddressingMode::Relative {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *pc = addr as u16;
            }
        }
    }
}