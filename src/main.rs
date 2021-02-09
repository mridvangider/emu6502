pub mod instruction;

use crate::instruction::*;

struct Cpu
{
    reg_a:      u8,
    reg_x:      u8,
    reg_y:      u8,
    
    pc:         u16,
    sp:         u8,
    status:     u8,

    memory:     Vec<u8>
}

fn main() {
    println!("Hello, world!");
}
