//! # emu6502::instruction::bitwise
//! 
//! This module contains the implementations of the following arithmetic functions:
//!     - and : AND memory with accumulator
//!     - ora : OR memory with accumulator
//!     - eor : XOR memory with one
//!     - asl : Arithmetic shift left one bit
//!     - lsr : Logical shift right one bit
//!     - rol : Rotate left one bit
//!     - ror : Rotate right one bit
use crate::util::{
    Operand,
    AddressingMode,
    FLAG_ZERO,
    FLAG_NEGATIVE,
    FLAG_CARRY,
};

use crate::cpu::*;

impl CPU {
    /// AND memory with accumulator
    /// 
    /// A & M -> A
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn and(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    self.reg_a &= *val;

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    let val = self.mem[addr];
                    self.reg_a &= val;

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            _ => {}
        }
    }

    /// OR memory with accumulator
    /// 
    /// A | M -> A
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn ora(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    self.reg_a |= *val;

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    let val = self.mem[addr];
                    self.reg_a |= val;

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            }
            _ => {}
        }
    }

    /// XOR memory with accumulator
    /// 
    /// A ^ M -> A
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn eor(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    self.reg_a ^= *val;

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    let val = self.mem[addr];
                    self.reg_a ^= val;

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            }
            _ => {}
        }
    }

    /// Arithmetic shift left one bit
    /// 
    /// C <- B7 B6 B5 B4 B3 B2 B1 B0 <- 0
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    ///     - Carry
    pub fn asl(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                if ( self.reg_a & (1 << 7)  ) > 0 {
                    self.stat |= FLAG_CARRY;
                } else {
                    self.stat &= !FLAG_CARRY;
                }

                self.reg_a = self.reg_a << 1;

                if self.reg_a == 0 {
                    self.stat |= FLAG_ZERO;
                }

                self.stat |= FLAG_NEGATIVE & self.reg_a;
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get_mut(addr) {
                        if *val & ( 1 << 7) > 0 {
                            self.stat |= FLAG_CARRY;
                        } else {
                            self.stat &= !FLAG_CARRY;
                        }
        
                        *val = *val << 1;
        
                        if *val == 0 {
                            self.stat |= FLAG_ZERO;
                        }
        
                        self.stat |= FLAG_NEGATIVE & *val;
                    }
                }
            }
            _ => {}
        }
    }

    /// Logical shift right one bit
    /// 
    /// 0 -> B7 B6 B5 B4 B3 B2 B1 B0 -> C
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    ///     - Carry
    pub fn lsr(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                if self.reg_a & 1 > 0 {
                    self.stat |= FLAG_CARRY;
                } else {
                    self.stat &= !FLAG_CARRY;
                }

                self.reg_a = self.reg_a >> 1;

                if self.reg_a == 0 {
                    self.stat |= FLAG_ZERO;
                }

                self.stat |= FLAG_NEGATIVE & self.reg_a;
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get_mut(addr) {
                        if *val & 1 > 0 {
                            self.stat |= FLAG_CARRY;
                        } else {
                            self.stat &= !FLAG_CARRY;
                        }
        
                        *val = *val >> 1;
        
                        if *val == 0 {
                            self.stat |= FLAG_ZERO;
                        }
        
                        self.stat |= FLAG_NEGATIVE & *val;
                    }
                }
            }
            _ => {}
        }
    }

    /// Rotate left one bit
    /// 
    /// C <- B7 B6 B5 B4 B3 B2 B1 B0 <- C
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    ///     - Carry
    pub fn rol(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                let old_carry = self.stat & FLAG_CARRY;

                if ( self.reg_a & (1 << 7)  ) > 0 {
                    self.stat |= FLAG_CARRY;
                } else {
                    self.stat &= !FLAG_CARRY;
                }

                self.reg_a = self.reg_a << 1;
                self.reg_a |= old_carry;

                if self.reg_a == 0 {
                    self.stat |= FLAG_ZERO;
                }

                self.stat |= FLAG_NEGATIVE & self.reg_a;
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get_mut(addr) {
                        let old_carry = self.stat & FLAG_CARRY;

                        if *val & ( 1 << 7) > 0 {
                            self.stat |= FLAG_CARRY;
                        } else {
                            self.stat &= !FLAG_CARRY;
                        }
        
                        *val = *val << 1;
                        *val |= old_carry;
        
                        if *val == 0 {
                            self.stat |= FLAG_ZERO;
                        }
        
                        self.stat |= FLAG_NEGATIVE & *val;
                    }
                }
            }
            _ => {}
        }
    }

    /// Rotate right one bit
    /// 
    /// C -> B7 B6 B5 B4 B3 B2 B1 B0 -> C
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    ///     - Carry
    pub fn ror(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                let old_carry = self.stat & FLAG_CARRY;

                if self.reg_a & 1 > 0 {
                    self.stat |= FLAG_CARRY;
                } else {
                    self.stat &= !FLAG_CARRY;
                }

                self.reg_a = self.reg_a >> 1;
                self.reg_a |= old_carry << 7;

                if self.reg_a == 0 {
                    self.stat |= FLAG_ZERO;
                }

                self.stat |= FLAG_NEGATIVE & self.reg_a;
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get_mut(addr) {
                        let old_carry = self.stat & FLAG_CARRY;
                        
                        if *val & 1 > 0 {
                            self.stat |= FLAG_CARRY;
                        } else {
                            self.stat &= !FLAG_CARRY;
                        }
        
                        *val = *val >> 1;
                        *val |= old_carry << 7;
        
                        if *val == 0 {
                            self.stat |= FLAG_ZERO;
                        }
        
                        self.stat |= FLAG_NEGATIVE & *val;
                    }
                }
            }
            _ => {}
        }
    }
}