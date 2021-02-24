//! This module contains the implementations of the following branch functions:
//!     - bcc : Branch on carry clear
//!     - bcs : Branch on carry set
//!     - bne : Branch on result not zero
//!     - beq : Branch on result zero
//!     - bpl : Branch on result plus
//!     - bmi : Branch on result minus
//!     - bvc : Branch on overflow clear
//!     - bvs : Branch on overflow set
use crate::util::{
    AddressingMode,
    Operand,
    FLAG_CARRY,
    FLAG_ZERO,
    FLAG_NEGATIVE,
    FLAG_OVERFLOW
};

use crate::cpu::*;

impl CPU {
    /// Branch on carry clear
    /// 
    /// branch on C = 0
    pub fn bcc(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_CARRY == 0 {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    /// Branch on carry set
    /// 
    /// branch on C = 1
    pub fn bcs(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_CARRY == FLAG_CARRY {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    /// Branch on result not zero
    /// 
    /// branch on Z = 0
    pub fn bne(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_ZERO == 0 {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    /// Branch on result zero
    /// 
    /// branch on Z = 1
    pub fn beq(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_ZERO == FLAG_ZERO {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    /// Branch on result plus
    /// 
    /// branch on N = 0
    pub fn bpl(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_NEGATIVE == 0 {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    /// Branch on result minus
    /// 
    /// branch on N = 1
    pub fn bmi(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_NEGATIVE == FLAG_NEGATIVE {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    /// Branch on overflow clear
    /// 
    /// branch on V = 0
    pub fn bvc(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_OVERFLOW == 0 {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    /// Branch on overflow set
    /// 
    /// branch on V = 1
    pub fn bvs(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_OVERFLOW == FLAG_OVERFLOW {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }
}