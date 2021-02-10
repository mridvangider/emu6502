use super::util::{
    AddressingMode,
    Operand,
    Memory,
    calculate_address,
    FLAG_BREAK,
    FLAG_CARRY,
    FLAG_DECIMAL,
    FLAG_INTDIS,
    FLAG_NEGATIVE,
    FLAG_OVERFLOW,
    FLAG_ZERO,

};

pub fn add(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                let old_carry = *stat & FLAG_CARRY;
                if u8::MAX - *reg_a >= *val {
                    *reg_a += *val;
                } else {
                    *reg_a = (*reg_a).wrapping_add(*val);
                    *stat |= FLAG_CARRY;
                }

                if old_carry == FLAG_CARRY {
                    if u8::MAX != *reg_a {
                        *reg_a += 1;
                        *stat &= !FLAG_CARRY;
                    } else {
                        *reg_a = (*reg_a).wrapping_add(1);
                        *stat |= FLAG_CARRY;
                    }
                }

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        },
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY | AddressingMode::ZeroPage | 
        AddressingMode::ZeroPageIndexedX |AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                *reg_a += mem[addr];
            }
        },
        _ => {}
    }
}