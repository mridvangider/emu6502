//! This module contains the implementations of the following set&clear functions:
//!     - clc : Clear carry
//!     - sec : Set carry
//!     - cld : Clear decimal
//!     - sed : Set decimal
//!     - cli : Clear interrupt
//!     - sei : Set interrupt
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
    pub fn clc (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat &= !FLAG_CARRY;
        }
    }

    pub fn sec (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat |= FLAG_CARRY;
        }
    }

    pub fn cld (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat &= !FLAG_DECIMAL;
        }
    }

    pub fn sed (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat |= FLAG_DECIMAL;
        }
    }

    pub fn cli (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat &= !FLAG_INTDIS;
        }
    }

    pub fn sei (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat |= FLAG_INTDIS;
        }
    }

    pub fn clv (&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.stat &= !FLAG_OVERFLOW;
        }
    }
}