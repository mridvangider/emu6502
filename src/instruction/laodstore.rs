use super::util::{
    Memory,
    Operand,
    AddressingMode,
    FLAG_NEGATIVE,
    FLAG_ZERO,
    calculate_address,
};

pub fn lda (reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                *reg_a = *val;

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        },
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                if let Option::Some(val) = mem.get(addr) {
                    *reg_a = *val;

                    if *reg_a == 0 {
                        *stat |= FLAG_ZERO;
                    }
    
                    *stat |= FLAG_NEGATIVE & *reg_a;
                }
            }
        }
        _ => {}
    }
}

pub fn ldx (_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                *reg_x = *val;

                if *reg_x == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_x;
            }
        },
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedY |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedY => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                if let Option::Some(val) = mem.get(addr) {
                    *reg_x = *val;

                    if *reg_x == 0 {
                        *stat |= FLAG_ZERO;
                    }
    
                    *stat |= FLAG_NEGATIVE & *reg_x;
                }
            }
        }
        _ => {}
    }
}

pub fn ldy (_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                *reg_y = *val;

                if *reg_y == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_y;
            }
        },
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                if let Option::Some(val) = mem.get(addr) {
                    *reg_y = *val;

                    if *reg_y == 0 {
                        *stat |= FLAG_ZERO;
                    }
    
                    *stat |= FLAG_NEGATIVE & *reg_y;
                }
            }
        }
        _ => {}
    }
}

pub fn sta (reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, _stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                if let Option::Some(val) = mem.get_mut(addr) {
                    *val = *reg_a;
                }
            }
        }
        _ => {}
    }
}

pub fn stx (_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, _stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Absolute | AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedY => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                if let Option::Some(val) = mem.get_mut(addr) {
                    *val = *reg_x;
                }
            }
        }
        _ => {}
    }
}

pub fn sty (_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, _stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Absolute | AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                if let Option::Some(val) = mem.get_mut(addr) {
                    *val = *reg_y;
                }
            }
        }
        _ => {}
    }
}