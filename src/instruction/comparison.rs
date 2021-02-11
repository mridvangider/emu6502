use super::util::{
    Operand,
    Memory,
    AddressingMode,
    calculate_address,
    FLAG_NEGATIVE,
    FLAG_OVERFLOW,
    FLAG_ZERO,
    FLAG_CARRY,
};

pub fn cmp(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                if *reg_a < *val {
                    *stat |= FLAG_NEGATIVE;
                } else if *reg_a > *val {
                    *stat |= FLAG_CARRY;
                } else {
                    *stat |= FLAG_ZERO;
                }
            }
        },
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndexedIndirect | 
        AddressingMode::IndirectIndexed => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                let val = mem[addr];
                if *reg_a < val {
                    *stat |= FLAG_NEGATIVE;
                } else if *reg_a > val {
                    *stat |= FLAG_CARRY;
                } else {
                    *stat |= FLAG_ZERO;
                }
            }
        }
        _ => {}
    }
}

pub fn cpx(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                if *reg_x < *val {
                    *stat |= FLAG_NEGATIVE;
                } else if *reg_x > *val {
                    *stat |= FLAG_CARRY;
                } else {
                    *stat |= FLAG_ZERO;
                }
            }
        },
        AddressingMode::Absolute | AddressingMode::ZeroPage => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                let val = mem[addr];
                if *reg_x < val {
                    *stat |= FLAG_NEGATIVE;
                } else if *reg_x > val {
                    *stat |= FLAG_CARRY;
                } else {
                    *stat |= FLAG_ZERO;
                }
            }
        }
        _ => {}
    }
}

pub fn cpy(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                if *reg_y < *val {
                    *stat |= FLAG_NEGATIVE;
                } else if *reg_y > *val {
                    *stat |= FLAG_CARRY;
                } else {
                    *stat |= FLAG_ZERO;
                }
            }
        },
        AddressingMode::Absolute | AddressingMode::ZeroPage => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                let val = mem[addr];
                if *reg_y < val {
                    *stat |= FLAG_NEGATIVE;
                } else if *reg_y > val {
                    *stat |= FLAG_CARRY;
                } else {
                    *stat |= FLAG_ZERO;
                }
            }
        }
        _ => {}
    }
}

pub fn bit(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Absolute | AddressingMode::ZeroPage => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                if let Option::Some(val) = mem.get(addr) {
                    *stat |= FLAG_NEGATIVE & ( *val & ( 1 << 6 ) );
                    *stat |= FLAG_OVERFLOW & ( *val & ( 1 << 5 ) );

                    if *reg_a & *val == 0 {
                        *stat |= FLAG_ZERO;
                    }
                }                
            }
        },
        _ => {}
    }
}