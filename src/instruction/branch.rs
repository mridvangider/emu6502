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
    pub fn bcc(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_CARRY == 0 {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    pub fn bcs(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_CARRY == FLAG_CARRY {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    pub fn bne(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_ZERO == 0 {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    pub fn beq(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_ZERO == FLAG_ZERO {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    pub fn bpl(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_NEGATIVE == 0 {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    pub fn bmi(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_NEGATIVE == FLAG_NEGATIVE {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

    pub fn bvc(&mut self, op : &Operand, mode: AddressingMode) {
        if self.stat & FLAG_OVERFLOW == 0 {
            if mode == AddressingMode::Relative {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    self.pc = addr as u16;
                }
            }
        }
    }

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