//! This module contains the implementations of the following miscellenaus functions:
//!     - brk : Force an interrupt
//!     - nop : Do nothing
use crate::util::{
    AddressingMode,
    Operand,
    FLAG_BREAK,
};
use crate::cpu::*;

impl CPU {
    /// Force an interrupt
    /// 
    /// 1 -> B
    /// 1 -> I
    /// 
    /// Effected flags:
    ///     - Break
    ///     - Interrupt
    pub fn brk (&mut self, _op : &Operand, _mode: AddressingMode) {
        if let Result::Err(_) = self.push_stack_word(self.pc, true) {
            return;
        }

        if let Result::Err(_) = self.push_stack_byte(self.stat | FLAG_BREAK) {
            return;
        }

        self.int = false;
        self.load_vector(0xFFFE, 0xFFFF);
    }

    /// Do nothing
    /// 
    /// Effected flags: None
    pub fn nop (&mut self, _op : &Operand, _mode : AddressingMode) {
        return;
    }
}