use super::super::util::{
    AddressingMode,
    Mnemonic,
    Operand,
    change_endianness,
    split_word,
    make_word,
    safe_remove,
};
use super::instruction_table::MNEMONICS;
use crate::errors::*;

pub fn decode_mnemonic<'a>(ocode: &u8) -> Option<&'a Mnemonic> {
    for mnem in MNEMONICS.iter() {
        if mnem.opcode == *ocode {
            return Some(mnem);
        }
    }
    return None;
}

pub fn encode_operand(op: &Operand) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    
    match op {
        Operand::Byte(b) => { ret.push(*b); },
        Operand::Word(w) => { ret.append(
                &mut (split_word(
                    &change_endianness(w)
                ).to_vec())
            ); 
        }
        _ => {}
    }
    
    return ret;
}

pub fn addr_mode_to_oprnd_size(mode: AddressingMode) -> usize {
    match mode {
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | 
        AddressingMode::Indirect | AddressingMode::AbsoluteIndexedY => 2,
        AddressingMode::Relative | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::ZeroPageIndexedY => 1,
        _ => 0
    }
}

pub fn get_operand(bytes: &mut Vec<u8>, mode: AddressingMode) -> Result<Operand, Err> {
    match addr_mode_to_oprnd_size(mode) {
        0 => {
            return Ok(
                Operand::Null
            );
        },
        1 => {
            return Ok(
                Operand::Byte(
                    safe_remove(bytes, 1)?
                )
            );
        },
        2 => {
            let (low, high): (u8,u8);
            low = safe_remove(bytes, 1)?;
            high = safe_remove(bytes, 1)?;

            return Ok(
                Operand::Word(
                    make_word(&low, &high)
                )
            )
        }
        _ => { return Err(ERR_NO_VALID_OPERAND); }
    }
}

pub fn find_mnemonic_by_name<'a> (name : &str) -> Result<&'a Mnemonic, Err> {
    for m in MNEMONICS.iter() {
        if name == m.name {
            return Ok(m);
        }
    }

    return Err(ERR_MNEMONIC_NOT_FOUND);
}

pub fn process_operand(op : &str, mode : AddressingMode) -> Result<Operand, Err> {
    
    match mode {
        AddressingMode::Absolute => {
            if op.len() == 5 {
                if op.starts_with('$') {
                    match &op[1..5].parse::<u16>() {
                        Ok(addr) => Ok(Operand::Word(*addr)),
                        Err(_) => Err(ERR_PARSE_ERROR)
                    }
                } else {
                    return Err(ERR_OPERAND_WRONG_FORMAT);
                }
            } else {
                return Err(ERR_OPERAND_SIZE_INVALID);
            }
        },
        AddressingMode::AbsoluteIndexedX => {
            if op.len() == 9 {                                  
                if op.starts_with("($") && op.ends_with(",X)") {
                    match &op[2..6].parse::<u16>() {
                        Ok(addr) => Ok(Operand::Word(*addr)),
                        Err(_) => Err(ERR_PARSE_ERROR)
                    }
                } else {
                    return Err(ERR_OPERAND_WRONG_FORMAT);
                }
            } else {
                return Err(ERR_OPERAND_SIZE_INVALID);
            }
        }
        _ => Ok(Operand::Null)
    }
}
/*
pub fn compile_line(code: &str) -> Result<Instruction,Err> {
    match code.lines().next() {
        Some(line) => {
            let words : Vec<&str> = line.split(' ').collect();
            let name = words[0].trim().to_uppercase();
            let mnem = find_mnemonic_by_name(&name)?;
    
            if mnem.addr_mode == AddressingMode::Implied || mnem.addr_mode == AddressingMode::Accumulator {
                if words.len() == 1 {
                    return Ok(
                        Instruction {
                            mnemonic: mnem,
                            operand: Operand::Null,
                        }
                    )
                } else {
                    return Err(ERR_WRONG_OPERAND_COUNT);
                }
            } else {
                if words.len() == 2 {
                    return Ok(
                        Instruction {
                            mnemonic: mnem,
                            operand: process_operand(
                                words[1].to_ascii_uppercase().as_str(), 
                                mnem.addr_mode
                            )?
                        }
                    )

                } else {
                    return Err(ERR_WRONG_OPERAND_COUNT);
                }
            }
        },
        None => Err(ERR_INVALID_LINE)
    }
}

pub fn decompile(ins: &Vec<Instruction>) -> String {
    let ret = String::new();
    return ret;
}*/