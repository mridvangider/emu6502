use super::util::{
    AddressingMode,
    Memory,
    Operand,
    push_stack_word,
    pop_stack_word,
    pop_stack_byte,
    calculate_address,
    make_word,
};

pub fn jmp (_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, _sp: &mut u8, _stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Absolute || mode == AddressingMode::Indirect {
        if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
            let new_pc = make_word(&mem[addr],&mem[addr+1]);
            *pc = new_pc.saturating_sub(1);
        }
    }
}

pub fn jsr (_reg_a: &mut u8, reg_x: &mut u8, reg_y: &mut u8, pc: &mut u16, sp: &mut u8, _stat: &mut u8, op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Absolute {
        if let Option::Some(addr) = calculate_address(op, reg_x, reg_y, pc, mem, mode) {
            if let Result::Err(_) = push_stack_word(*pc, sp, mem, true) {
                return;
            }

            let new_pc = make_word(&mem[addr], &mem[addr+1]);
            *pc = new_pc.saturating_sub(1);
        }
    }
}

pub fn rts (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, pc: &mut u16, sp: &mut u8, _stat: &mut u8, _op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        if let Result::Ok(new_pc) = pop_stack_word(sp, mem, false) {
            *pc = new_pc;
        }
    }
}

pub fn rti (_reg_a: &mut u8, _reg_x: &mut u8, _reg_y: &mut u8, _pc: &mut u16, sp: &mut u8, stat: &mut u8, _op: &Operand, mem: &mut Memory, mode: AddressingMode) {
    if mode == AddressingMode::Implied {
        if let Result::Ok(new_stat) = pop_stack_byte(sp, mem) {
            *stat = new_stat;
        }
    }
}