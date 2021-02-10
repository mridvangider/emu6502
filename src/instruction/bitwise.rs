use super::util::{
    Operand,
    Memory,
    AddressingMode,
    calculate_address,
    FLAG_ZERO,
    FLAG_NEGATIVE,
};

pub fn and(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                *reg_a &= *val;

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        },
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                let val = mem[addr];
                *reg_a &= val;

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        }
        _ => {}
    }
}

pub fn ora(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                *reg_a |= *val;

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        },
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                let val = mem[addr];
                *reg_a |= val;

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        }
        _ => {}
    }
}

pub fn eor(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                *reg_a ^= *val;

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        },
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                let val = mem[addr];
                *reg_a ^= val;

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        }
        _ => {}
    }
}