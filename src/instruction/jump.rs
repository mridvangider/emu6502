//! This module contains the implementations of the following jump functions:
//!     - jmp : Jump to a new location by changing the value of the program counter
//!     - jsr : Jump to a subrouine
//!     - rts : Return from the subroutine to the point where it called with `JSR`
//!     - rti : Return from interrupt
use crate::util::{
    AddressingMode,
    Operand,
    make_word,
};
use crate::cpu::*;

impl CPU {
    /// Jump to a new location by changing the value of the program counter
    /// 
    /// M -> PC
    /// 
    /// Effected flags: None
    pub fn jmp (&mut self, op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Absolute || mode == AddressingMode::Indirect {
            if let Option::Some(addr) = self.calculate_address(op, mode) {
                self.pc = make_word(&self.mem[addr],&self.mem[addr+1]);
            }
        }
    }

    /// Jump to a subroutine. The address of the next instruction is pushed to the stack
    /// 
    /// PCH -> SP
    /// PCL -> SP+1
    /// M -> PC
    /// 
    /// Effected flags: None
    pub fn jsr (&mut self, op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Absolute {
            if let Option::Some(addr) = self.calculate_address(op, mode) {
                if let Result::Err(_) = self.push_stack_word(self.pc, true) {
                    return;
                }

                self.pc = make_word(&self.mem[addr], &self.mem[addr+1]);
            }
        }
    }

    /// Return from the subroutine to the point where it called `JSR`
    /// 
    /// SP -> PCL
    /// SP-1 -> PCH
    /// 
    /// Effected flags: None
    pub fn rts (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if let Result::Ok(new_pc) = self.pop_stack_word(false) {
                self.pc = new_pc;
            }
        }
    }

    /// Return from interrupt
    /// 
    /// SP -> SR
    /// SP-1 -> PCL
    /// SP-1 -> PCH
    /// 
    /// Effected flags: All
    pub fn rti (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            if let Result::Ok(new_stat) = self.pop_stack_byte() {
                self.stat = new_stat;
            }

            if let Result::Ok(new_pc) = self.pop_stack_word(false) {
                self.pc = new_pc;
            }
        }
    }
}