use crate::util::{
    AddressingMode,
    Memory,
    Operand,
    push_stack_word,
};
use crate::cpu::*;

impl CPU {
    pub fn brk (&mut self, op : &Operand, mode: AddressingMode) -> {
        if let Result::Err(_) = self.push_stack_word(self.pc, true) {
            return;
        }

        if let Result::Err(_) = self.push_stack_byte(self.stat | FLAG_BREAK) {
            return;
        }

        self.load_vector(0xFFFE, 0xFFFF);
    }
}