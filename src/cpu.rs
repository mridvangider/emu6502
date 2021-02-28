use crate::util::*;
use super::errors::*;
use super::instruction::compile::*;
use super::instruction::instruction_table::MNEMONICS;

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
    /// Create a new CPU object with zero filled memory.
    pub fn new() -> CPU {
        let mut mem : Vec<u8> = Vec::new();
        mem.fill(0);
        
        let mut ret = CPU {
            reg_a   : 0,
            reg_x   : 0,
            reg_y   : 0,

            pc      : 0,
            stat    : 0,
            sp      : 0,

            mem     : mem,
            int     : true,
        };
        ret.reset();
        
        ret
    }

    /// Create a new CPU object with the provided memory object.
    /// 
    /// Note that the returning CPU object takes the ownership of
    /// the memory object.
    pub fn from(mem : Memory) -> CPU {
        let mut ret = CPU {
            reg_a   : 0,
            reg_x   : 0,
            reg_y   : 0,

            pc      : 0,
            stat    : 0,
            sp      : 0,

            mem     : mem,
            int     : true,
        };
        ret.reset();
        
        ret
    }

    /// Simulates a cpu cycle. 
    /// 
    /// This is different from the way 6502 cycle actually works. 
    /// Usually, instructions requires multiple cycles to complete. 
    /// Here, we complete everything in a single cycle for simplicity.
    pub fn cycle(&mut self) -> Result<(),Err> {
        let ptr = self.pc;
        self.pc = self.pc.saturating_add(1);

        let opcode = safe_remove(&mut self.mem, ptr as usize)?;
        for mnem in MNEMONICS.iter() {
            if mnem.opcode == opcode {
                let op = get_operand(&mut self.mem, mnem.addr_mode)?;
                let f = mnem.ofunc;
                f(self, &op, mnem.addr_mode);
            }
        }

        if ptr == 0xFFFF {
            return Ok(());
        }
        return Ok(());
    }

    /// Reads a word from the addresses low,high into the program counter.
    /// 
    /// Example: Suppose we have 
    /// 
    ///  0xFFF0     00 00 00 00 00 00 00 00    00 00 00 00 00 AB CD 00
    /// 
    /// Then 
    /// 
    /// `load_vector(0xFFFD,0xFFFE)` 
    /// 
    /// puts **ABCD** into the program counter.
    pub fn load_vector(&mut self, low : u16, high : u16) {
        let l = self.mem[low as usize];
        let h = self.mem[high as usize];
        self.pc = make_word(&l, &h);
    }

    /// Simulates the RESET signal.
    /// 
    /// Loads the vector at 0xFFFC,0xFFFD into the program counter.
    pub fn reset(&mut self) {
        self.load_vector(0xFFFC, 0xFFFD);
    }

    /// Simulates the NMI signal.
    /// 
    /// Pushes the program counter and the processor status register 
    /// onto the stack, disables interrupts and the loads vector at 0xFFFA,0xFFFB.
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

    /// Simulates the IRQ signal.
    /// 
    /// Pushes the program counter and the processor status register 
    /// onto the stack, disables interrupts and loads the vector at 0xFFFE,0xFFFF.
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

    /// From provided operand, returns a *usize* according to the addresing mode.
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

    /// Pushes a byte onto the stack
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

    /// Pushes a word onto the stack.
    /// 
    /// First the low byte, then the second byte is pushed. If `little_endian` is true,
    /// the order is reversed.
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

    /// Pops a byte from the top of the stack
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

    /// Pops a word from the stack.
    /// 
    /// First the high byte, then the low byte is popped. If `big_endian` is true,
    /// the order is reversed.
    pub fn pop_stack_word(&mut self,big_endian: bool) -> Result<u16,Err> {
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

        if big_endian {
            ret = change_endianness(&ret);
        }
        
        return Ok(ret);
    }


}