use crate::util::{
    Operand,
    AddressingMode,
    FLAG_NEGATIVE,
    FLAG_OVERFLOW,
    FLAG_ZERO,
    FLAG_CARRY,
};

use crate::cpu::*;

impl CPU {
    pub fn cmp(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    if self.reg_a < *val {
                        self.stat |= FLAG_NEGATIVE;
                    } else if self.reg_a > *val {
                        self.stat |= FLAG_CARRY;
                    } else {
                        self.stat |= FLAG_ZERO;
                    }
                }
            },
            AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY |
            AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::IndexedIndirect | 
            AddressingMode::IndirectIndexed => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    let val = self.mem[addr];
                    if self.reg_a < val {
                        self.stat |= FLAG_NEGATIVE;
                    } else if self.reg_a > val {
                        self.stat |= FLAG_CARRY;
                    } else {
                        self.stat |= FLAG_ZERO;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn cpx(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    if self.reg_x < *val {
                        self.stat |= FLAG_NEGATIVE;
                    } else if self.reg_x > *val {
                        self.stat |= FLAG_CARRY;
                    } else {
                        self.stat |= FLAG_ZERO;
                    }
                }
            },
            AddressingMode::Absolute | AddressingMode::ZeroPage => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    let val = self.mem[addr];
                    if self.reg_x < val {
                        self.stat |= FLAG_NEGATIVE;
                    } else if self.reg_x > val {
                        self.stat |= FLAG_CARRY;
                    } else {
                        self.stat |= FLAG_ZERO;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn cpy(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Immediate => {
                if let Operand::Byte(val) = op {
                    if self.reg_y < *val {
                        self.stat |= FLAG_NEGATIVE;
                    } else if self.reg_y > *val {
                        self.stat |= FLAG_CARRY;
                    } else {
                        self.stat |= FLAG_ZERO;
                    }
                }
            },
            AddressingMode::Absolute | AddressingMode::ZeroPage => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    let val = self.mem[addr];
                    if self.reg_y < val {
                        self.stat |= FLAG_NEGATIVE;
                    } else if self.reg_y > val {
                        self.stat |= FLAG_CARRY;
                    } else {
                        self.stat |= FLAG_ZERO;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn bit(&mut self, op : &Operand, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute | AddressingMode::ZeroPage => {
                if let Option::Some(addr) = self.calculate_address(op, mode) {
                    if let Option::Some(val) = self.mem.get(addr) {
                        self.stat |= FLAG_NEGATIVE & ( *val & ( 1 << 6 ) );
                        self.stat |= FLAG_OVERFLOW & ( *val & ( 1 << 5 ) );

                        if self.reg_a & *val == 0 {
                            self.stat |= FLAG_ZERO;
                        }
                    }                
                }
            },
            _ => {}
        }
    }
}