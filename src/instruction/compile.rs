use super::util::{
    Instruction,
    Mnemonic,
    Operand,
    change_endianness,
    split_word,
};
use super::instruction_table::MNEMONICS;

fn find_mnemonic(ocode: &u8) -> Option<&Mnemonic> {
    for mnem in MNEMONICS.iter() {
        if mnem.opcode == *ocode {
            return Some(mnem);
        }
    }
    return None;
}

fn compile_operand(op: &Vec<Operand>) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    
    for o in op {
        match o {
            Operand::Byte(b) => { ret.push(*b); },
            Operand::Word(w) => { ret.append(
                    &mut (split_word(
                        &change_endianness(w)
                    ).to_vec())
                ); 
            }
            _ => {}
        }
    }
    return ret;
}

pub fn assemble(ins: &Vec<Instruction>) -> Vec<u8> {
    let mut ret: Vec<u8>  = Vec::new();
    for i in ins.iter() {
        if let Option::Some(mnem) = find_mnemonic(&(i.opcode)) {
            let mut data = compile_operand(&(i.operands));
            ret.push(mnem.opcode);
            ret.append(&mut data);
        }
    }
    return ret;
}

/*
pub fn disassemble(bytes: &Vec<u8>) -> Vec<Instruction> {
    let ret: Vec<Instruction> = Vec::new();
    return ret;
}

pub fn compile(code: &str) -> Vec<Instruction> {
    let ret: Vec<Instruction> = Vec::new();
    return ret;
}

pub fn decompile(ins: &Vec<Instruction>) -> String {
    let ret = String::new();
    return ret;
}*/