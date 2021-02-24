//! This module contains the implementations of the following load&store functions:
//!     - lda : Load accumulator with memory
//!     - ldx : Load index X with memory
//!     - ldy : Load index Y with memory
//!     - sta : Store accumulator in memory
//!     - stx : Store index X in memory
//!     - sty : Store index Y in memory
use crate::util::{
    Operand,
    AddressingMode,
    FLAG_NEGATIVE,
    FLAG_ZERO,
};

use crate::cpu::*;

impl CPU {
    /// Load accumulator with memory
    /// 
    /// M -> A
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn lda (&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    self.reg_a = *val;

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get(addr) {
                        self.reg_a = *val;

                        if self.reg_a == 0 {
                            self.stat |= FLAG_ZERO;
                        }
        
                        self.stat |= FLAG_NEGATIVE & self.reg_a;
                    }
                }
            }
            _ => {}
        }
    }

    /// Load index X with memory
    /// 
    /// M -> X
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn ldx (&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    self.reg_x = *val;

                    if self.reg_x == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_x;
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedY |
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedY => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get(addr) {
                        self.reg_x = *val;

                        if self.reg_x == 0 {
                            self.stat |= FLAG_ZERO;
                        }
        
                        self.stat |= FLAG_NEGATIVE & self.reg_x;
                    }
                }
            }
            _ => {}
        }
    }

    /// Load index Y with memory
    /// 
    /// M -> Y
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn ldy (&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    self.reg_y = *val;

                    if self.reg_y == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_y;
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX |
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get(addr) {
                        self.reg_y = *val;

                        if self.reg_y == 0 {
                            self.stat |= FLAG_ZERO;
                        }
        
                        self.stat |= FLAG_NEGATIVE & self.reg_y;
                    }
                }
            }
            _ => {}
        }
    }

    /// Store accumulator in memory
    /// 
    /// A -> M
    /// 
    /// Effected flags: None
    pub fn sta (&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get_mut(addr) {
                        *val = self.reg_a;
                    }
                }
            }
            _ => {}
        }
    }

    /// Store index X in memory
    /// 
    /// X -> M
    /// 
    /// Effected flags: Node
    pub fn stx (&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute | AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedY => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get_mut(addr) {
                        *val = self.reg_x;
                    }
                }
            }
            _ => {}
        }
    }

    /// Store index Y in memory
    /// 
    /// y -> M
    /// 
    /// Effected flags: None
    pub fn sty (&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute | AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get_mut(addr) {
                        *val = self.reg_y;
                    }
                }
            }
            _ => {}
        }
    }
}