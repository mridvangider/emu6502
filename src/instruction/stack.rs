use crate::util::{
    AddressingMode,
    Operand,
};

use crate::cpu::*;

impl CPU {
    pub fn pha (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if self.push_stack_byte(self.reg_a) == Result::Ok(()) {
                return;
            }
        }
    }

    pub fn pla (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if let Result::Ok(val) = self.pop_stack_byte() {
                self.reg_a = val;
            }
        }
    }

    pub fn php (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if self.push_stack_byte(self.stat) == Result::Ok(()) {
                return;
            }
        }
    }

    pub fn plp (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if let Result::Ok(val) = self.pop_stack_byte() {
                self.stat = val;
            }
        }
    }
}