//! This module contains the implementations of the following transfer functions:
//!     - tax : Transfer accumulator to index X
//!     - txa : Transfer index X to accumulator
//!     - tay : Transfer accumulator to index Y
//!     - tya : Transfer index Y to accumulator
//!     - tsx : Transfer stack pointer to index X
//!     - txs : Transfer index X to stack pointer
use crate::util::{
    AddressingMode,
    Operand,
    FLAG_ZERO,
    FLAG_NEGATIVE
};

use crate::cpu::*;

impl CPU {
    /// Transfer accumulator to index X
    /// 
    /// A -> X
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn tax(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_x = self.reg_a;

            if self.reg_x == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_x;
        }
    }

    /// Transfer index X to accumulator
    /// 
    /// X -> A
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn txa(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_a = self.reg_x;
            
            if self.reg_a == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_a;
        }
    }

    /// Transfer accumulator to index Y
    /// 
    /// A -> Y
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn tay(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_y = self.reg_a;

            if self.reg_y == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_y;
        }
    }

    /// Transfer index Y to accumulator
    /// 
    /// Y -> A
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn tya(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_a = self.reg_y;
            
            if self.reg_a == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_a;
        }
    }

    /// Transfer stack pointer to index X
    /// 
    /// SP -> X
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn tsx(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_x = self.sp;

            if self.reg_x == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_x;
        }
    }

    /// Transfer index X to stack pointer
    /// 
    /// X -> SP
    /// 
    /// Effected flags: None
    pub fn txs(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.sp = self.reg_x;
        }
    }
}