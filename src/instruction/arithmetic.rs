//! # emu6502::instruction::aritmetic
//! 
//! This module contains the implementations of the following arithmetic functions:
//!     - adc : Add memory to accumulator with carry
//!     - sbc : Substract memory from accumulator with carry
//!     - inc : Increment memory by one
//!     - inx : Increment Index X by one
//!     - iny : Increment Index Y by one
//!     - dec : Decrement memory by one
//!     - dex : Decrement Index X by one
//!     - dey : Decrement Index Y by one

use super::super::util::{
    AddressingMode,
    Operand,
    FLAG_CARRY,
    FLAG_NEGATIVE,
    FLAG_ZERO,
};
use super::super::cpu::*;
impl CPU {
    /// Add memory to accumulator with carry
    /// 
    /// A + M + C -> A
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Overflow(*)
    ///     - Zero
    ///     - Carry
    /// 
    /// ***Note:*** In this implementation, the Overflow flag is not effected
    pub fn adc(&mut self, op : &Operand, mode : AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    let old_carry = self.stat & FLAG_CARRY;
                    if u8::MAX - self.reg_a >= *val {
                        self.reg_a += *val;
                        self.stat &= !FLAG_CARRY;
                    } else {
                        self.reg_a = (self.reg_a).wrapping_add(*val);
                        self.stat |= FLAG_CARRY;
                    }

                    if old_carry == FLAG_CARRY {
                        if u8::MAX != self.reg_a {
                            self.reg_a += 1;
                            self.stat &= !FLAG_CARRY;
                        } else {
                            self.reg_a = (self.reg_a).wrapping_add(1);
                            self.stat |= FLAG_CARRY;
                        }
                    }

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY | AddressingMode::ZeroPage | 
            AddressingMode::ZeroPageIndexedX |AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    let val: u8 = self.mem[addr];
                    let old_carry = self.stat & FLAG_CARRY;
                    if u8::MAX - self.reg_a >= val {
                        self.reg_a += val;
                        self.stat &= !FLAG_CARRY;
                    } else {
                        self.reg_a = (self.reg_a).wrapping_add(val);
                        self.stat |= FLAG_CARRY;
                    }

                    if old_carry == FLAG_CARRY {
                        if u8::MAX != self.reg_a {
                            self.reg_a += 1;
                            self.stat &= !FLAG_CARRY;
                        } else {
                            self.reg_a = (self.reg_a).wrapping_add(1);
                            self.stat |= FLAG_CARRY;
                        }
                    }

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            _ => {}
        }
    }

    /// Subtract memory from accumulator with carry
    /// 
    /// A - M - ~C -> A
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Overflow(*)
    ///     - Zero
    ///     - Carry
    /// 
    /// ***Note***: In this implementation, the Overflow flag is not effected
    pub fn sbc(&mut self, op : &Operand, mode : AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    let old_carry = self.stat & FLAG_CARRY;
                    if self.reg_a >= *val {
                        self.reg_a -= *val;
                        self.stat &= !FLAG_CARRY;
                    } else {
                        self.reg_a = (self.reg_a).wrapping_sub(*val);
                        self.stat |= FLAG_CARRY;
                    }

                    if old_carry == FLAG_CARRY {
                        if u8::MIN != self.reg_a {
                            self.reg_a -= 1;
                            self.stat &= !FLAG_CARRY;
                        } else {
                            self.reg_a = (self.reg_a).wrapping_sub(1);
                            self.stat |= FLAG_CARRY;
                        }
                    }

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY | AddressingMode::ZeroPage | 
            AddressingMode::ZeroPageIndexedX |AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    let val: u8 = self.mem[addr];
                    let old_carry = self.stat & FLAG_CARRY;
                    if self.reg_a >= val {
                        self.reg_a -= val;
                        self.stat &= !FLAG_CARRY; 
                    } else {
                        self.reg_a = (self.reg_a).wrapping_sub(val);
                        self.stat |= FLAG_CARRY;
                    }

                    if old_carry == FLAG_CARRY {
                        if u8::MIN != self.reg_a {
                            self.reg_a -= 1;
                            self.stat &= !FLAG_CARRY;
                        } else {
                            self.reg_a = (self.reg_a).wrapping_sub(1);
                            self.stat |= FLAG_CARRY;
                        }
                    }

                    if self.reg_a == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.reg_a;
                }
            },
            _ => {}
        }
    }

    /// Increment memory by one
    /// 
    /// M + 1 -> M
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn inc(&mut self, op : &Operand, mode : AddressingMode) {
        match mode {
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.mem[addr] = self.mem[addr].wrapping_add(1);
                    
                    if self.mem[addr] == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.mem[addr];
                }
            },
            _ => {}
        }
    }

    /// Increment index X by one
    /// 
    /// X + 1 -> X
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn inx(&mut self, _op : &Operand, mode : AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_x = self.reg_x.wrapping_add(1);

            if self.reg_x == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_x;
        }
    }

    /// Increment index Y by one
    /// 
    /// Y + 1 -> Y
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn iny(&mut self, _op : &Operand, mode : AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_y = self.reg_y.wrapping_add(1);

            if self.reg_y == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_y;
        }
    }

    /// Decrement memory by one
    /// 
    /// M - 1 -> M
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn dec(&mut self, op : &Operand, mode : AddressingMode) {
        match mode {
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.mem[addr] = self.mem[addr].wrapping_sub(1);
                    
                    if self.mem[addr] == 0 {
                        self.stat |= FLAG_ZERO;
                    }

                    self.stat |= FLAG_NEGATIVE & self.mem[addr];
                }
            },
            _ => {}
        }
    }

    /// Decrement index X by one
    /// 
    /// X - 1 -> X
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn dex(&mut self, _op : &Operand, mode : AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_x = self.reg_x.wrapping_sub(1);

            if self.reg_x == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_x;
        }
    }

    /// Decrement index Y by one
    /// 
    /// Y - 1 -> Y
    /// 
    /// Effected flags:
    ///     - Negative
    ///     - Zero
    pub fn dey(&mut self, _op : &Operand, mode : AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_y = self.reg_y.wrapping_sub(1);

            if self.reg_y == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_y;
        }
    }
}