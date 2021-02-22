use crate::util::{
    Operand,
    AddressingMode,
    FLAG_NEGATIVE,
    FLAG_ZERO,
};

use crate::cpu::*;

impl CPU {
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