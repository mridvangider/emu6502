use super::util::{
    AddressingMode,
    Memory,
    Operand,
    push_stack_byte,
    pop_stack_byte,
};

pub fn pha (reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, sp: &mut u8, _stat: &mut u8, _op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        if push_stack_byte(*reg_a, sp, mem) == Result::Ok(()) {
            return;
        }
    }
}

pub fn pla (reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, sp: &mut u8, _stat: &mut u8, _op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        if let Result::Ok(val) = pop_stack_byte(sp, mem) {
            *reg_a = val;
        }
    }
}

pub fn php (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, sp: &mut u8, stat: &mut u8, _op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        if push_stack_byte(*stat, sp, mem) == Result::Ok(()) {
            return;
        }
    }
}

pub fn plp (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, sp: &mut u8, stat: &mut u8, _op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        if let Result::Ok(val) = pop_stack_byte(sp, mem) {
            *stat = val;
        }
    }
}