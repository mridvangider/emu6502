//! This module contains the implementations of the following stack functions:
//!     - pha : Push accumulator to stack
//!     - pla : Pull accumulator from stack
//!     - php : Push processor status to stack
//!     - plp : Pull processor status from stack
use crate::util::{
    AddressingMode,
    Operand,
};

use crate::cpu::*;

impl CPU {
    /// Push accumulator to stack
    /// 
    /// Effected flags: None
    pub fn pha (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if self.push_stack_byte(self.reg_a) == Result::Ok(()) {
                return;
            }
        }
    }

    /// Pull accumulator from stack
    /// 
    /// Effected flags: None
    pub fn pla (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if let Result::Ok(val) = self.pop_stack_byte() {
                self.reg_a = val;
            }
        }
    }

    /// Push processor status to stack
    /// 
    /// Effected flags: None
    pub fn php (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if self.push_stack_byte(self.stat) == Result::Ok(()) {
                return;
            }
        }
    }

    /// Pull processor status from stack
    /// 
    /// Effected flags: All
    pub fn plp (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if let Result::Ok(val) = self.pop_stack_byte() {
                self.stat = val;
            }
        }
    }
}