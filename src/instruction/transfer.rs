use crate::util::{
    AddressingMode,
    Operand,
    FLAG_ZERO,
    FLAG_NEGATIVE
};

use crate::cpu::*;

impl CPU {
    pub fn tax(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_x = self.reg_a;

            if self.reg_x == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_x;
        }
    }

    pub fn txa(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_a = self.reg_x;
            
            if self.reg_a == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_a;
        }
    }

    pub fn tay(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_y = self.reg_a;

            if self.reg_y == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_y;
        }
    }

    pub fn tya(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_a = self.reg_y;
            
            if self.reg_a == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_a;
        }
    }

    pub fn tsx(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.reg_x = self.sp;

            if self.reg_x == 0 {
                self.stat |= FLAG_ZERO;
            }

            self.stat |= FLAG_NEGATIVE & self.reg_x;
        }
    }

    pub fn txs(&mut self, _op : &Operand, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            self.sp = self.reg_x;
        }
    }
}