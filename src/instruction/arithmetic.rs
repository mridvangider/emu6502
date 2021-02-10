use super::util::{
    AddressingMode,
    Operand,
    Memory,
    calculate_address,
    FLAG_CARRY,
    FLAG_NEGATIVE,
    FLAG_ZERO,

};

pub fn add(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                let old_carry = *stat & FLAG_CARRY;
                if u8::MAX - *reg_a >= *val {
                    *reg_a += *val;
                    *stat &= !FLAG_CARRY;
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
                let val: u8 = mem[addr];
                let old_carry = *stat & FLAG_CARRY;
                if u8::MAX - *reg_a >= val {
                    *reg_a += val;
                    *stat &= !FLAG_CARRY;
                } else {
                    *reg_a = (*reg_a).wrapping_add(val);
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
        _ => {}
    }
}

pub fn sub(reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => {
            if let Operand::Byte(val) = op {
                let old_carry = *stat & FLAG_CARRY;
                if *reg_a >= *val {
                    *reg_a -= *val;
                    *stat &= !FLAG_CARRY;
                } else {
                    *reg_a = (*reg_a).wrapping_sub(*val);
                    *stat |= FLAG_CARRY;
                }

                if old_carry == FLAG_CARRY {
                    if u8::MIN != *reg_a {
                        *reg_a -= 1;
                        *stat &= !FLAG_CARRY;
                    } else {
                        *reg_a = (*reg_a).wrapping_sub(1);
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
                let val: u8 = mem[addr];
                let old_carry = *stat & FLAG_CARRY;
                if *reg_a >= val {
                    *reg_a -= val;
                    *stat &= !FLAG_CARRY; 
                } else {
                    *reg_a = (*reg_a).wrapping_sub(val);
                    *stat |= FLAG_CARRY;
                }

                if old_carry == FLAG_CARRY {
                    if u8::MIN != *reg_a {
                        *reg_a -= 1;
                        *stat &= !FLAG_CARRY;
                    } else {
                        *reg_a = (*reg_a).wrapping_sub(1);
                        *stat |= FLAG_CARRY;
                    }
                }

                if *reg_a == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & *reg_a;
            }
        },
        _ => {}
    }
}

pub fn inc(_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    match mode {
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX => {
            if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
                mem[addr] = mem[addr].wrapping_add(1);
                
                if mem[addr] == 0 {
                    *stat |= FLAG_ZERO;
                }

                *stat |= FLAG_NEGATIVE & mem[addr];
            }
        },
        _ => {}
    }
}

pub fn inx(_reg_a: &mut u8, reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, _sp: &mut u8, stat: &mut u8, _op: &Operand, _mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        *reg_x = reg_x.wrapping_add(1);

        if *reg_x == 0 {
            *stat |= FLAG_ZERO;
        }

        *stat |= FLAG_NEGATIVE & *reg_x;
    }
}