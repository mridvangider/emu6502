use crate::util::{
    Operand,
    AddressingMode,
    change_endianness,
    split_word,
    make_word,
};
use super::errors::*;

pub struct CPU {
    
    pub reg_a   : u8,
    pub reg_x   : u8,
    pub reg_y   : u8,

    pub pc      : u16,
    pub stat    : u8,
    pub sp      : u8,

    pub mem     : Vec<u8>,

    pub int     : bool,
}

impl CPU {
    pub fn load_vector(&mut self, low : u16, high : u16) {
        let l = self.mem[low as usize];
        let h = self.mem[high as usize];
        self.pc = make_word(&l, &h);
    }

    pub fn reset(&mut self) {
        self.load_vector(0xFFFC, 0xFFFD);
    }

    pub fn nmi(&mut self) {
        if let Result::Err(_) = self.push_stack_word(self.pc, true) {
            return;
        }

        if let Result::Err(_) = self.push_stack_byte(self.stat) {
            return;
        }

        self.int = false;
        self.load_vector(0xFFFA, 0xFFFB);
    }

    pub fn irq(&mut self) {
        if let Result::Err(_) = self.push_stack_word(self.pc, true) {
            return;
        }

        if let Result::Err(_) = self.push_stack_byte(self.stat) {
            return;
        }

        self.int = false;
        self.load_vector(0xFFFE, 0xFFFF);
    }

    pub fn calculate_address(&self, op: &Operand, mode: AddressingMode) -> Option<usize> {
        let mut ret: usize;
        match mode {
            AddressingMode::Absolute => {
                if let Operand::Word(addr) = op {
                    return Some(change_endianness(addr) as usize);
                } else {
                    return None;
                }
            },
            AddressingMode::AbsoluteIndexedX => {
                if let Operand::Word(addr) = op {
                    ret = change_endianness(addr) as usize;
                    ret += self.reg_x as usize;
                    return Some(ret);
                } else {
                    return None;
                }
            },
            AddressingMode::AbsoluteIndexedY => {
                if let Operand::Word(addr) = op {
                    ret = change_endianness(addr) as usize;
                    ret += self.reg_y as usize;
                    return Some(ret);
                } else {
                    return None;
                }
            },
            AddressingMode::Indirect => {
                if let Operand::Word(addr) = op {
                    ret = change_endianness(addr) as usize;
                    ret = self.mem[ret] as usize ;
                    return Some(ret);
                } else {
                    return None;
                }
            },
            AddressingMode::IndexedIndirect => {
                if let Operand::Byte(addr) = op {
                    ret = ( *addr as usize ) + ( self.reg_x as usize );
                    ret = self.mem[ret] as usize;
                    return Some(ret);
                } else {
                    return None;
                }
            },
            AddressingMode::IndirectIndexed => {
                if let Operand::Byte(addr) = op {
                    ret = ( self.mem[*addr as usize] + self.reg_y ) as usize;
                    return Some(ret);
                } else {
                    return None;
                }
            },
            AddressingMode::Relative => {
                if let Operand::Byte(addr) = op {
                    ret = ( ( self.pc as i32) + ( *addr as i32 ) ) as usize;
                    return Some(ret);
                } else {
                    return None;
                }
            },
            AddressingMode::ZeroPageIndexedX => {
                if let Operand::Byte(addr) = op {
                    ret = ( ( *addr + self.reg_x) % 0xff ) as usize;
                    return Some(ret);
                } else {
                    return None;
                }
            },
            AddressingMode::ZeroPageIndexedY => {
                if let Operand::Byte(addr) = op {
                    ret = ( ( *addr + self.reg_y) % 0xff ) as usize;
                    return Some(ret);
                } else {
                    return None;
                }
            }
            _ => return None
        }
    }

    pub fn push_stack_byte(&mut self, val: u8) -> Result<(),Err> {
        if self.sp > 0 {
            self.sp -= 1;
            let addr: usize = 0x0100 + self.sp as usize;
            self.mem[addr] = val;
            return Ok(());
        } else {
            return Err(ERR_STACK_FULL);
        }
    }

    pub fn push_stack_word(&mut self, word: u16, little_endian: bool) -> Result<(),Err> {
        let mut val = word;
        
        if little_endian {
            val = change_endianness(&val);
        }

        let split: [u8;2] = split_word(&val);
        
        if let Result::Err(e) = self.push_stack_byte(split[0]) {
            return Err(e);
        }
        if let Result::Err(e) = self.push_stack_byte(split[1]) {
            return Err(e);
        }

        return Ok(());
    }

    pub fn pop_stack_byte(&mut self) -> Result<u8,Err> {
        if self.sp < 0xFF {
            let addr = 0x0100 + self.sp as usize;
            let ret = self.mem[addr];
            self.sp += 1;
            return Ok(ret);
        } else {
            return Err(ERR_STACK_EMPTY);
        }
    }

    pub fn pop_stack_word(&mut self,little_endian: bool) -> Result<u16,Err> {
        let mut ret: u16;
        let (low, high) : (u8,u8);
        match self.pop_stack_byte() {
            Ok(val) => low = val,
            Err(e) => return Err(e),
        }
        
        match self.pop_stack_byte() {
            Ok(val) => high = val,
            Err(e) => return Err(e),
        }

        ret = make_word(&low, &high);

        if little_endian {
            ret = change_endianness(&ret);
        }
        
        return Ok(ret);
    }
}