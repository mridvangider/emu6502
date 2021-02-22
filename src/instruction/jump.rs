use crate::util::{
    AddressingMode,
    Operand,
    make_word,
};
use crate::cpu::*;

impl CPU {
    pub fn jmp (&mut self, op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Absolute || mode == AddressingMode::Indirect {
            if let Option::Some(addr) = self.calculate_address(op, mode) {
                let new_pc = make_word(&self.mem[addr],&self.mem[addr+1]);
                self.pc = new_pc.saturating_sub(1);
            }
        }
    }

    pub fn jsr (&mut self, op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Absolute {
            if let Option::Some(addr) = self.calculate_address(op, mode) {
                if let Result::Err(_) = self.push_stack_word(self.pc, true) {
                    return;
                }

                let new_pc = make_word(&self.mem[addr], &self.mem[addr+1]);
                self.pc = new_pc.saturating_sub(1);
            }
        }
    }

    pub fn rts (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if let Result::Ok(new_pc) = self.pop_stack_word(false) {
                self.pc = new_pc;
            }
        }
    }

    pub fn rti (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if let Result::Ok(new_stat) = self.pop_stack_byte() {
                self.stat = new_stat;
            }
        }
    }
}