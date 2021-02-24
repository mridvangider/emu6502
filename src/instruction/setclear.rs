//! This module contains the implementations of the following set&clear functions:
//!     - clc : Clear carry
//!     - sec : Set carry
//!     - cld : Clear decimal
//!     - sed : Set decimal
//!     - cli : Clear interrupt disable
//!     - sei : Set interrupt disable
//!     - clv : Clear overflow
use crate::util::{
    AddressingMode,
    Operand,
    FLAG_CARRY,
    FLAG_DECIMAL,
    FLAG_INTDIS,
    FLAG_OVERFLOW,
};

use crate::cpu::*;

impl CPU {
    /// Clear carry
    /// 
    /// 0 -> C
    /// 
    /// Effected flags:
    ///     - Carry
    pub fn clc (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat &= !FLAG_CARRY;
        }
    }

    /// Set carry
    /// 
    /// 1 -> C
    /// 
    /// Effected flags:
    ///     - Carry
    pub fn sec (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat |= FLAG_CARRY;
        }
    }

    /// Clear decimal
    /// 
    /// 0 -> D
    /// 
    /// Effected flags:
    ///     - Decimal
    pub fn cld (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat &= !FLAG_DECIMAL;
        }
    }

    /// Set decimal
    /// 
    /// 1 -> D
    /// 
    /// Effected flags:
    ///     - Decimal
    pub fn sed (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat |= FLAG_DECIMAL;
        }
    }

    /// Clear interrupt disable
    /// 
    /// 0 -> I
    /// 
    /// Effected flags:
    ///     - Interrupt Disable
    pub fn cli (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat &= !FLAG_INTDIS;
        }
    }

    /// Set interrupt disable
    /// 
    /// 1 -> I
    /// 
    /// Effected flags:
    ///     - Interrupt Disable
    pub fn sei (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat |= FLAG_INTDIS;
        }
    }

    /// Clear overflow
    /// 
    /// 0 -> V
    /// 
    /// Effected flags:
    ///     - Overflow
    pub fn clv (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat &= !FLAG_OVERFLOW;
        }
    }
}